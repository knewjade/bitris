#pragma once

#include <experimental/simd>

#include "rows.hpp"
#include "pieces.hpp"
#include "rotate.hpp"
#include "kicks.hpp"
#include "bits.hpp"
#include "data.hpp"
#include "free_spaces.hpp"

namespace stdx = std::experimental;

namespace s {
    template<typename Data, Shape Shape>
    struct searcher {
        using data_t = data<Data>;
        using type = typename data_t::type;
        static constexpr auto N = free_spaces<Data, Shape>::N;

        template<typename U>
        static constexpr std::array<Data, N * 10> to(const std::array<typename data<U>::type, N> &boards) {
            alignas(32) std::array<Data, N * 10> array{};
            static_for<N>([&][[gnu::always_inline]](auto index) {
                data<U>::template to<Data>(boards[index]).copy_to(&array[index * 10], stdx::vector_aligned);
            });
            return array;
        }

        template<typename U>
        static constexpr std::array<Data, N * 10> search2(
            const std::array<Data, 10> &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const Data reachable_rows
        ) {
            const auto board_data = data_t::template load<U>(board);
            const auto goals = searcher<U, Shape>::begin(
                board_data, spawn_orientation, spawn_cx, spawn_cy, reachable_rows
            );
            return to<U>(goals);
        }

        static constexpr std::array<Data, N * 10> search(
            const std::array<Data, 10> &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            const auto used_rows = bits<Data>::used_rows(board);
            const auto [top_y, reachable_rows] = rows::spawn_bits(used_rows, spawn_cy);

            if (top_y < 14) {
                return search2<uint16_t>(board, spawn_orientation, spawn_cx, spawn_cy, reachable_rows);
            }

            if (top_y < 31) {
                return search2<uint32_t>(board, spawn_orientation, spawn_cx, spawn_cy, reachable_rows);
            }

            return search2<uint64_t>(board, spawn_orientation, spawn_cx, spawn_cy, reachable_rows);
        }

        static constexpr std::array<type, N> lock(
            const std::array<type, N> &all_free_space,
            const std::array<type, N> &all_reachable
        ) {
            std::array<type, N> all_goal{};
            static_for<N>([&][[gnu::always_inline]](auto index) {
                all_goal[index] = ~data_t::template shift_up<1>(all_free_space[index]) & all_reachable[index];
            });
            return all_goal;
        }

        static constexpr std::array<type, N> reach(
            const std::array<type, N> &all_free_space,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const Data reachable_rows
        ) {
            auto all_reachable = std::array<type, N>{};
            static_for<N>([&][[gnu::always_inline]](auto index) {
                if (index == spawn_orientation) {
                    all_reachable[index] = data_t::make_spawn(
                        reachable_rows,
                        all_free_space[index],
                        spawn_cx,
                        spawn_cy
                    );
                } else {
                    all_reachable[index] = data_t::make_zero();
                }
            });
            return all_reachable;
        }

        static constexpr type move(const auto &reachable, const auto &free_space) {
            const auto right = data_t::template shift_right<1>(reachable);
            const auto left = data_t::template shift_left<1>(reachable);
            const auto down = data_t::template shift_down<1>(reachable);
            return (reachable | right | left | down) & free_space;
        }

        template<Rotation Rotation>
        static constexpr type rotate2(
            const Orientation src_orientation,
            const type &src_reachable,
            const type &dest_free_space
        ) {
            switch (src_orientation) {
                case Orientation::North: {
                    constexpr auto piece = Piece{Shape, Orientation::North};
                    return rotate3<piece, Rotation>(src_reachable, dest_free_space);
                }
                case Orientation::East: {
                    constexpr auto piece = Piece{Shape, Orientation::East};
                    return rotate3<piece, Rotation>(src_reachable, dest_free_space);
                }
                case Orientation::South: {
                    constexpr auto piece = Piece{Shape, Orientation::South};
                    return rotate3<piece, Rotation>(src_reachable, dest_free_space);
                }
                case Orientation::West: {
                    constexpr auto piece = Piece{Shape, Orientation::West};
                    return rotate3<piece, Rotation>(src_reachable, dest_free_space);
                }
            }
            std::unreachable();
        }

        template<Piece FromPiece, Rotation Rotation>
        static constexpr type rotate3(
            const type &src_reachable,
            const type &dest_free_space
        ) {
            auto src_candidates = src_reachable;
            auto dest_reachable = data_t::make_zero();

            constexpr auto offsets = get_offsets<FromPiece, Rotation>();
            static_for_until<offsets>([&]<Offset offset>[[gnu::always_inline]]() {
                const auto shift_forward = data_t::template shift<offset>(src_candidates);
                dest_reachable = dest_reachable | shift_forward;
                const auto shift_backward = data_t::template shift<-offset>(dest_free_space);
                src_candidates = (~shift_backward) & src_candidates;
                return all_of(src_candidates == 0);
            });

            return dest_reachable & dest_free_space;
        }

        static constexpr std::array<type, N> begin(
            const type &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const Data reachable_rows
        ) {
            static_assert(N == 1 || N == 4);

            const auto free_space_block = ~board;
            const auto all_free_space = free_spaces<Data, Shape>::get(free_space_block);

            auto all_reachable = reach(
                all_free_space, spawn_orientation, spawn_cx, spawn_cy, reachable_rows
            );

            if constexpr (N == 4) {
                auto needs_update = std::bitset<N>((1U << N) - 1);
                auto current_index = static_cast<size_t>(spawn_orientation);

                while (needs_update.any()) {
                    static_for<N>([&][[gnu::always_inline]](auto _) {
                        // check
                        if (!needs_update[current_index]) {
                            current_index = (current_index + 1) % N;
                            return;
                        }
                        needs_update[current_index] = false;

                        // move
                        while (true) {
                            const auto &reachable = all_reachable[current_index];
                            const auto next = move(reachable, all_free_space[current_index]);
                            if (all_of(next == reachable)) {
                                break;
                            }
                            all_reachable[current_index] = next;
                        }

                        // rotate
                        const auto from_orientation = static_cast<Orientation>(current_index);

                        constexpr auto rotations = std::array{Rotation::Cw, Rotation::Ccw};
                        static_for<rotations>([&]<Rotation rotation>[[gnu::always_inline]]() {
                            // TODO orientaion テンプレートか
                            const auto to_orientation = rotate(from_orientation, rotation);
                            const auto dest_orientation_index = static_cast<size_t>(to_orientation);

                            const auto &reachable = all_reachable[dest_orientation_index];
                            const auto &free_spaces = all_free_space[dest_orientation_index];

                            const auto found_dest_reachable = rotate2<rotation>(
                                to_orientation, reachable, free_spaces
                            );

                            const auto dest_reachable = reachable | found_dest_reachable;

                            if (any_of(reachable != dest_reachable)) {
                                all_reachable[dest_orientation_index] = dest_reachable;
                                needs_update[dest_orientation_index] = true;
                            }
                        });

                        current_index = (current_index + 1) % N;
                    });
                }
            } else {
                constexpr size_t index = 0;

                // move
                while (true) {
                    const auto &reachable = all_reachable[index];
                    const auto next = move(reachable, all_free_space[index]);
                    if (all_of(next == reachable)) {
                        break;
                    }
                    all_reachable[index] = next;
                }
            }

            return lock(all_free_space, all_reachable);
        }
    };
}

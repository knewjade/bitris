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

    private:
        template<typename U>
        static constexpr std::array<Data, N * 10> search_casted(
            const std::array<Data, 10> &board,
            const Orientation spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            const auto board_u = data_t::template load<U>(board);

            const auto goals = searcher<U, Shape>::execute(
                board_u, spawn_orientation, spawn_cx, spawn_cy
            );

            alignas(32) std::array<Data, N * 10> array{};
            static_for<N>([&][[gnu::always_inline]](auto index) {
                data<U>::template to<Data>(goals[index]).copy_to(&array[index * 10], stdx::vector_aligned);
            });
            return array;
        }

    public:
        static constexpr std::array<Data, N * 10> search(
            const std::array<Data, 10> &board,
            const Orientation spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            const auto top_y = rows::top_y(board, spawn_cy);

            if (top_y < 6) {
                return search_casted<uint8_t>(board, spawn_orientation, spawn_cx, spawn_cy);
            }

            if (top_y < 14) {
                return search_casted<uint16_t>(board, spawn_orientation, spawn_cx, spawn_cy);
            }

            if (top_y < 30) {
                return search_casted<uint32_t>(board, spawn_orientation, spawn_cx, spawn_cy);
            }

            return search_casted<uint64_t>(board, spawn_orientation, spawn_cx, spawn_cy);
        }

        static constexpr std::array<type, N> execute(
            const type &board,
            const Orientation spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            if constexpr (N == 1) {
                constexpr auto orientation = Orientation::North;
                return execute<orientation>(
                    board, spawn_cx, spawn_cy
                );
            }

            switch (spawn_orientation) {
                case Orientation::North: {
                    constexpr auto orientation = Orientation::North;
                    return execute<orientation>(
                        board, spawn_cx, spawn_cy
                    );
                }
                case Orientation::East: {
                    constexpr auto orientation = Orientation::East;
                    return execute<orientation>(
                        board, spawn_cx, spawn_cy
                    );
                }
                case Orientation::South: {
                    constexpr auto orientation = Orientation::South;
                    return execute<orientation>(
                        board, spawn_cx, spawn_cy
                    );
                }
                case Orientation::West: {
                    constexpr auto orientation = Orientation::West;
                    return execute<orientation>(
                        board, spawn_cx, spawn_cy
                    );
                }
            }
            std::unreachable();
        }

        template<Orientation SpawnOrientation>
        [[gnu::always_inline]]
        static constexpr std::array<type, N> spawn(
            const std::array<type, N> &all_free_space,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            const auto spawn_orientation_index = static_cast<size_t>(SpawnOrientation);
            auto all_reachable = std::array<type, N>{};
            static_for<N>([&][[gnu::always_inline]](auto index) {
                if (index == spawn_orientation_index) {
                    all_reachable[index] = data_t::make_spawn(
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

    private:
        [[gnu::always_inline]]
        static constexpr type move(const auto &reachable, const auto &free_space) {
            const auto right = data_t::template shift_right<1>(reachable);
            const auto left = data_t::template shift_left<1>(reachable);
            const auto down = data_t::template shift_down<1>(reachable);
            return (reachable | right | left | down) & free_space;
        }

        [[gnu::always_inline]]
        static consteval std::array<Orientation, 4> orientation_order(const Orientation orientation) {
            static_assert(N == 4);

            switch (orientation) {
                case Orientation::North:
                    return {Orientation::North, Orientation::East, Orientation::South, Orientation::West};
                case Orientation::East:
                    return {Orientation::East, Orientation::South, Orientation::West, Orientation::North};
                case Orientation::South:
                    return {Orientation::South, Orientation::West, Orientation::North, Orientation::East};
                case Orientation::West:
                    return {Orientation::West, Orientation::North, Orientation::East, Orientation::South};
            }

            std::unreachable();
        }

        [[gnu::always_inline]]
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

        template<Orientation SpawnOrientation>
        static constexpr std::array<type, N> execute(
            const type &board,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            static_assert(N == 1 || N == 4);

            const auto free_space_block = ~board;
            const auto all_free_space = free_spaces<Data, Shape>::get(free_space_block);

            auto all_reachable = spawn<SpawnOrientation>(all_free_space, spawn_cx, spawn_cy);

            if constexpr (N == 4) {
                constexpr auto rotate = []<Orientation Orientation, Rotation Rotation>[[gnu::always_inline]](
                    const type &src_reachable,
                    const type &dest_free_space
                ) {
                    auto src_candidates = src_reachable;
                    auto dest_reachable = data_t::make_zero();

                    constexpr auto offsets = get_offsets<{Shape, Orientation}, Rotation>();
                    static_for_until<offsets>([&]<Offset offset>[[gnu::always_inline]]() {
                        const auto shift_forward = data_t::template shift<offset>(src_candidates);
                        dest_reachable = dest_reachable | shift_forward;
                        const auto shift_backward = data_t::template shift<-offset>(dest_free_space);
                        src_candidates = (~shift_backward) & src_candidates;
                        return all_of(src_candidates == 0);
                    });

                    return dest_reachable & dest_free_space;
                };

                auto needs_update = std::bitset<N>().flip();
                while (needs_update.any()) {
                    static_for<orientation_order(SpawnOrientation)>([&]<Orientation Orientation>[[gnu::always_inline]]() {
                        // check
                        auto current_index = static_cast<size_t>(Orientation);
                        if (!needs_update[current_index]) {
                            return;
                        }
                        needs_update.reset(current_index);

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
                        const auto reachable_for_rotate = all_reachable[current_index] & data_t::template make_square<(bits<Data>::full >> 2)>();
                        constexpr auto rotations = std::array{Rotation::Cw, Rotation::Ccw};
                        static_for<rotations>([&]<Rotation Rotation>[[gnu::always_inline]]() {
                            constexpr auto to_orientation = rotate_to(Orientation, Rotation);
                            constexpr auto dest_orientation_index = static_cast<size_t>(to_orientation);

                            const auto found_dest_reachable = rotate.template operator()<Orientation, Rotation>(
                                reachable_for_rotate, all_free_space[dest_orientation_index]
                            );

                            const auto dest_reachable = all_reachable[dest_orientation_index] | found_dest_reachable;

                            if (any_of(all_reachable[dest_orientation_index] != dest_reachable)) {
                                all_reachable[dest_orientation_index] = dest_reachable;
                                needs_update.set(dest_orientation_index);
                            }
                        });
                    });
                }
            } else {
                constexpr size_t index = 0;
                std::cout << "start" << std::endl;

                // move
                while (true) {
                    std::cout << "move " << std::endl;
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

#pragma once

#include <utility>
#include <bitset>

#include "rows.hpp"
#include "pieces.hpp"
#include "rotate.hpp"
#include "kicks.hpp"
#include "data.hpp"
#include "free_spaces.hpp"

namespace s {
    template<typename Data, Shape Shape>
    struct searcher {
        using bits_t = bits<Data>;
        using data_t = data<Data>;
        using type = typename data_t::type;
        static constexpr auto N = free_spaces<Data, Shape>::N;

    private:
        template<typename U>
        static constexpr std::array<Data, N * 10> search_casted(
                const std::array<Data, 10> &board_aligned,
                const Orientation spawn_orientation,
                const uint8_t spawn_cx,
                const uint8_t spawn_cy
        ) {
            const auto board_u = data_t::template load<U>(board_aligned);

            const auto goals = searcher<U, Shape>::execute(
                    board_u, spawn_orientation, spawn_cx, spawn_cy
            );

            alignas(32) std::array<Data, N * 10> array{};
            static_for_t<N>([&]<size_t SrcIndex> [[gnu::always_inline]]() {
                const auto converted = data<U>::template to<Data>(goals[SrcIndex]);
                static_for_t<10>([&]<size_t DestIndex> [[gnu::always_inline]]() {
                    array[SrcIndex * 10 + DestIndex] = converted[DestIndex];
                });
            });
            return array;
        }

    public:
        static constexpr std::array<Data, N * 10> search(
                const std::array<Data, 10> &board_aligned,
                const Orientation spawn_orientation,
                const uint8_t spawn_cx,
                const uint8_t spawn_cy
        ) {
            const auto top_y = rows::top_y(board_aligned, spawn_cy);

            if (top_y < 6) {
                return search_casted<uint8_t>(board_aligned, spawn_orientation, spawn_cx, spawn_cy);
            }

            if (top_y < 14) {
                return search_casted<uint16_t>(board_aligned, spawn_orientation, spawn_cx, spawn_cy);
            }

            return search_casted<uint32_t>(board_aligned, spawn_orientation, spawn_cx, spawn_cy);
        }

        static constexpr std::array<type, N> execute(
                const type &board,
                const Orientation spawn_orientation,
                const uint8_t spawn_cx,
                const uint8_t spawn_cy
        ) {
            if constexpr (N == 1) {
                constexpr auto orientation = Orientation::North;
                return execute < orientation > (
                        board, spawn_cx, spawn_cy
                );
            }

            switch (spawn_orientation) {
                case Orientation::North: {
                    constexpr auto orientation = Orientation::North;
                    return execute < orientation > (
                            board, spawn_cx, spawn_cy
                    );
                }
                case Orientation::East: {
                    constexpr auto orientation = Orientation::East;
                    return execute < orientation > (
                            board, spawn_cx, spawn_cy
                    );
                }
                case Orientation::South: {
                    constexpr auto orientation = Orientation::South;
                    return execute < orientation > (
                            board, spawn_cx, spawn_cy
                    );
                }
                case Orientation::West: {
                    constexpr auto orientation = Orientation::West;
                    return execute < orientation > (
                            board, spawn_cx, spawn_cy
                    );
                }
            }
            std::unreachable();
        }

    private:
        template<Orientation SpawnOrientation>
        [[gnu::always_inline]]
        static constexpr std::array<type, N> spawn(
                const std::array<type, N> &all_free_space,
                const uint8_t spawn_cx,
                const uint8_t spawn_cy
        ) {
            const auto spawn_orientation_index = static_cast<size_t>(SpawnOrientation);
            auto all_reachable = std::array<type, N>{};
            static_for_t<N>([&]<size_t Index> [[gnu::always_inline]]() {
                if (Index == spawn_orientation_index) {
                    all_reachable[Index] = data_t::make_spawn(
                            all_free_space[Index],
                            spawn_cx,
                            spawn_cy
                    );
                } else {
                    all_reachable[Index] = data_t::make_zero();
                }
            });
            return all_reachable;
        }

        [[gnu::always_inline]]
        static constexpr type move(const auto &reachable, const auto &free_space) {
            const auto right = data_t::template shift_right<1>(reachable);
            const auto left = data_t::template shift_left<1>(reachable);
            if constexpr (16 < bits_t::bit_size) {
                // 下移動はコストが比較して低く、横より縦の回数が大きくなりやすいため、2ブロック移動する
                const auto down1 = data_t::template shift_down<1>(reachable);
                const auto down2 = data_t::template shift_down<1>(down1 & free_space);
                return (reachable | right | left | down1 | down2) & free_space;
            } else {
                const auto down1 = data_t::template shift_down<1>(reachable);
                return (reachable | right | left | down1) & free_space;
            }
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
            static_for_t<N>([&]<size_t Index> [[gnu::always_inline]]() {
                all_goal[Index] = ~data_t::template shift_up<1>(all_free_space[Index]) & all_reachable[Index];
            });
            return all_goal;
        }

        template<Orientation Orientation>
        [[gnu::always_inline]]
        static constexpr void move_and_rotate(
                std::bitset<N> &needs_update,
                const std::array<type, N> &all_free_space,
                std::array<type, N> &all_reachable,
                std::array<type, N> &rotated_already
        ) {
            constexpr auto rotate = []<Rotation Rotation> [[gnu::always_inline]](
                    const type &src_reachable,
                    const type &dest_free_space
            ) {
                auto src_candidates = src_reachable;
                auto dest_reachable = data_t::make_zero();

                constexpr auto offsets = get_offsets<{Shape, Orientation}, Rotation>();
                static_assert(offsets.size() == 5);

                do {
                    // offset[0]
                    {
                        constexpr auto offset = offsets[0];
                        const auto shift_forward = data_t::template shift<offset>(src_candidates);
                        dest_reachable = dest_reachable | shift_forward;
                        const auto shift_backward = data_t::template shift<-offset>(dest_free_space);
                        src_candidates = (~shift_backward) & src_candidates;
                        if (data_t::is_equal_to(src_candidates, 0)) {
                            break;
                        }
                    }
                    // offset[1]
                    {
                        constexpr auto offset = offsets[1];
                        const auto shift_forward = data_t::template shift<offset>(src_candidates);
                        dest_reachable = dest_reachable | shift_forward;
                        const auto shift_backward = data_t::template shift<-offset>(dest_free_space);
                        src_candidates = (~shift_backward) & src_candidates;
                        if (data_t::is_equal_to(src_candidates, 0)) {
                            break;
                        }
                    }
                    // offset[2]
                    {
                        constexpr auto offset = offsets[2];
                        const auto shift_forward = data_t::template shift<offset>(src_candidates);
                        dest_reachable = dest_reachable | shift_forward;
                        const auto shift_backward = data_t::template shift<-offset>(dest_free_space);
                        src_candidates = (~shift_backward) & src_candidates;
                        if (data_t::is_equal_to(src_candidates, 0)) {
                            break;
                        }
                    }
                    // offset[3]
                    {
                        constexpr auto offset = offsets[3];
                        const auto shift_forward = data_t::template shift<offset>(src_candidates);
                        dest_reachable = dest_reachable | shift_forward;
                        const auto shift_backward = data_t::template shift<-offset>(dest_free_space);
                        src_candidates = (~shift_backward) & src_candidates;
                        if (data_t::is_equal_to(src_candidates, 0)) {
                            break;
                        }
                    }
                    // offset[4]
                    {
                        constexpr auto offset = offsets[4];
                        const auto shift_forward = data_t::template shift<offset>(src_candidates);
                        dest_reachable = dest_reachable | shift_forward;
                    }
                } while (false);

                return dest_reachable & dest_free_space;
            };

            // check
            constexpr auto current_index = static_cast<size_t>(Orientation);
            if (!needs_update[current_index]) {
                return;
            }
            needs_update.reset(current_index);

            // move
            while (true) {
                const auto &reachable = all_reachable[current_index];
                const auto next = move(reachable, all_free_space[current_index]);
                if (data_t::is_equal_to(next, reachable)) {
                    break;
                }
                all_reachable[current_index] = next;
            }

            // rotate
            const auto reachable_for_rotate = all_reachable[current_index] &
                                              data_t::template make_square<(bits<Data>::full >> 2)>() &
                                              ~rotated_already[current_index];
            rotated_already[current_index] |= reachable_for_rotate;
            constexpr auto rotations = std::array{Rotation::Cw, Rotation::Ccw};
            static_for_t<rotations>([&]<Rotation Rotation> [[gnu::always_inline]]() {
                constexpr auto to_orientation = rotate_to(Orientation, Rotation);
                constexpr auto dest_orientation_index = static_cast<size_t>(to_orientation);

                const auto found_dest_reachable = rotate.template operator()<Rotation>(
                        reachable_for_rotate, all_free_space[dest_orientation_index]
                );

                const auto dest_reachable =
                        all_reachable[dest_orientation_index] | found_dest_reachable;
                if (data_t::is_not_equal_to(all_reachable[dest_orientation_index], dest_reachable)) {
                    all_reachable[dest_orientation_index] = dest_reachable;
                    needs_update.set(dest_orientation_index);
                }
            });
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
            auto rotated_already = std::array<type, N>{};

            if constexpr (N == 4) {
                auto needs_update = std::bitset<N>().flip();
                while (needs_update.any()) {
                    static_for_t<orientation_order(SpawnOrientation)>(
                            [&]<Orientation Orientation>() {
                                move_and_rotate<Orientation>(
                                        needs_update, all_free_space, all_reachable, rotated_already
                                );
                            });
                }
            } else {
                constexpr size_t index = 0;
                // std::cout << "start" << std::endl;

                // move
                while (true) {
                    // std::cout << "move " << std::endl;
                    const auto &reachable = all_reachable[index];
                    const auto next = move(reachable, all_free_space[index]);

                    if (data_t::is_equal_to(next, reachable)) {
                        break;
                    }
                    all_reachable[index] = next;
                }
            }

            return lock(all_free_space, all_reachable);
        }
    };
}

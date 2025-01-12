#pragma once

#include <experimental/simd>

#include "rows.hpp"
#include "pieces.hpp"
#include "bits.hpp"
#include "data.hpp"
#include "free_spaces.hpp"

namespace stdx = std::experimental;

namespace s {
    template<typename T, Shape shape>
    struct searcher {
        using data_t = data<T>;
        using type = typename data_t::type;
        static constexpr auto N = free_spaces<T, shape>::N;

        template<typename U>
        static constexpr std::array<T, N * 10> to(const std::array<typename data<U>::type, N> &boards) {
            alignas(32) std::array<T, N * 10> array{};
            static_for<N>([&][[gnu::always_inline]](auto index) {
                data<U>::template to<T>(boards[index]).copy_to(&array[index * 10], stdx::vector_aligned);
            });
            return array;
        }

        template<typename U>
        static constexpr std::array<T, N * 10> search2(
            const std::array<T, 10> &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const T reachable_rows
        ) {
            const auto board_data = data_t::template load<U>(board);
            const auto goals = searcher<U, shape>::begin(
                board_data, spawn_orientation, spawn_cx, spawn_cy, reachable_rows
            );
            return to<U>(goals);
        }

        static constexpr std::array<T, N * 10> search(
            const std::array<T, 10> &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
        ) {
            const auto used_rows = bits<T>::used_rows(board);
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
            const T reachable_rows
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
                    all_reachable[index] = data_t::template make_square<0>();
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

        static constexpr std::array<type, N> begin(
            const type &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const T reachable_rows
        ) {
            static_assert(N == 1 || N == 4);

            const auto free_space_block = ~board;
            const auto all_free_space = free_spaces<T, shape>::get(free_space_block);

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
                            // TODO 変数に入れる？
                            all_reachable[current_index] = next;
                        }

                        // TODO 回転
                        // needs_update[current_index] = true;

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

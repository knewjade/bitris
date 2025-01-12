#pragma once

#include <experimental/simd>

#include "rows.hpp"
#include "pieces.hpp"
#include "bits.hpp"
#include "data.hpp"
#include "free_spaces.hpp"

namespace stdx = std::experimental;

namespace s {
    template<typename T>
    struct searcher {
        template<typename U>
        static constexpr std::array<U, 10> to(const typename data<T>::type &board) {
            alignas(32) std::array<U, 10> array{};
            data<T>::template to<U>(board).copy_to(&array[0], stdx::vector_aligned);
            return array;
        }

        static constexpr std::array<T, 10> search3(
            const std::array<T, 10> &board,
            const uint8_t spawn_shape,
            const uint8_t spawn_orientation,
            const uint8_t spawn_bx,
            const uint8_t spawn_by
        ) {
            switch (static_cast<Shape>(spawn_shape)) {
                case Shape::T:
                    return search<Shape::T>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
                case Shape::S:
                    return search<Shape::S>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
                case Shape::Z:
                    return search<Shape::Z>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
                case Shape::L:
                    return search<Shape::L>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
                case Shape::J:
                    return search<Shape::J>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
                case Shape::I:
                    return search<Shape::I>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
                case Shape::O:
                    return search<Shape::O>(
                        board, spawn_orientation, spawn_bx, spawn_by
                    );
            }
            std::unreachable();
        }

        template<Shape shape>
        static constexpr std::array<T, 10> search(
            const std::array<T, 10> &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_bx,
            const uint8_t spawn_by
        ) {
            const auto used_rows = bits<T>::used_rows(board);
            const auto [top_y, reachable_rows] = rows::spawn_bits(used_rows, spawn_by);

            if (top_y < 14) {
                using U = uint16_t;
                const data<U>::type board_data = data<T>::template load<U>(board);
                const data<U>::type goal = searcher<U>::begin<shape>(
                    board_data, spawn_orientation,
                    spawn_bx, spawn_by, reachable_rows
                );
                return searcher<U>::to<T>(goal);
            }

            assert(top_y < 31); {
                using U = uint32_t;
                const data<U>::type board_data = data<T>::template load<U>(board);
                const data<U>::type goal = searcher<U>::begin<shape>(
                    board_data, spawn_orientation,
                    spawn_bx, spawn_by, reachable_rows
                );

                return searcher<U>::to<T>(goal);
            }
        }

        using data_t = data<T>;
        using type = typename data_t::type;

        static constexpr type free_space(
            const type &free_space_block,
            const uint8_t spawn_piece,
            const uint8_t spawn_orientation
        ) {

        }

        struct Constants {
            size_t orientation_length;
        };

        static consteval Constants constants(const Shape shape) {
            if (shape == Shape::O) {
                return {1};
            }
            return {4};
        }

        template<Shape shape, size_t N = constants(shape).orientation_length>
        static constexpr type begin(
            const type &board,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const T reachable_rows
        ) {
            const auto free_space_block = ~board;

            const auto free_space = free_spaces<T, Shape::O>::north(free_space_block);

            auto reachable = data_t::make_spawn(
                reachable_rows,
                free_space,
                spawn_cx,
                spawn_cy
            );

            while (true) {
                const auto right = data_t::template shift_right<1>(reachable);
                const auto left = data_t::template shift_left<1>(reachable);
                const auto down = data_t::template shift_down<1>(reachable);

                const auto next = (reachable | right | left | down) & free_space;

                if (all_of(next == reachable)) {
                    break;
                }

                reachable = next;
            }

            const auto goal = ~data_t::template shift_up<1>(free_space) & reachable;

            return goal;
        }

        static constexpr type begin2(
            const type &board,
            const uint8_t spawn_piece,
            const uint8_t spawn_orientation,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy,
            const T reachable_rows
        ) {
            const auto free_space_block = ~board;

            const auto free_space = free_spaces<T, Shape::O>::north(free_space_block);

            auto reachable = data_t::make_spawn(
                reachable_rows,
                free_space,
                spawn_cx,
                spawn_cy
            );

            while (true) {
                const auto right = data_t::template shift_right<1>(reachable);
                const auto left = data_t::template shift_left<1>(reachable);
                const auto down = data_t::template shift_down<1>(reachable);

                const auto next = (reachable | right | left | down) & free_space;

                if (all_of(next == reachable)) {
                    break;
                }

                reachable = next;
            }

            const auto goal = ~data_t::template shift_up<1>(free_space) & reachable;

            return goal;
        }
    };
}

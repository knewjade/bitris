#pragma once

#include <cstddef>

#include "pieces.hpp"

template<typename, Shape>
struct kicks {
};

template<typename Data>
struct kicks<Data, Shape::T> {
    static constexpr size_t N = 1;

    static constexpr std::array<type, N> get(const type &free_space_block) {
        return {north(free_space_block)};
    }

    static constexpr type north(const type &free_space_block) {
        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
        return l1a & data_t::template shift_down<1, true>(l1a);
    }
};

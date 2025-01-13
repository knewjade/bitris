#pragma once

#include "pieces.hpp"
#include "data.hpp"

template<typename, Shape>
struct free_spaces {
};

template<typename Data>
struct free_spaces<Data, Shape::O> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 1;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        return {north(free_space_block)};
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
        return l1a & data_t::template shift_down<1, true>(l1a);
    }
};

template<typename Data>
struct free_spaces<Data, Shape::T> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 4;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        return {
            north(free_space_block),
            east(free_space_block),
            south(free_space_block),
            west(free_space_block),
        };
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        return l1 & r1 & d1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return l1 & d1 & u1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type south(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return l1 & r1 & u1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type west(const type &free_space_block) {
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return r1 & d1 & u1 & free_space_block;
    }
};

template<typename Data>
struct free_spaces<Data, Shape::L> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 4;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        return {
            north(free_space_block),
            east(free_space_block),
            south(free_space_block),
            west(free_space_block),
        };
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto l1d1 = data_t::template shift_down<1, true>(l1);
        return l1 & r1 & l1d1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto l1au1 = data_t::template shift_up<1>(l1a);
        return l1a & d1 & l1au1;
    }

    [[gnu::always_inline]]
    static constexpr type south(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto r1u1 = data_t::template shift_up<1>(r1);
        return l1 & r1 & r1u1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type west(const type &free_space_block) {
        const auto r1a = free_space_block & data_t::template shift_right<1>(free_space_block);
        const auto r1ad1 = data_t::template shift_down<1, true>(r1a);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return r1a & r1ad1 & u1;
    }
};

template<typename Data>
struct free_spaces<Data, Shape::J> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 4;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        return {
            north(free_space_block),
            east(free_space_block),
            south(free_space_block),
            west(free_space_block),
        };
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto r1d1 = data_t::template shift_down<1, true>(r1);
        return l1 & r1 & r1d1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
        const auto l1ad1 = data_t::template shift_down<1, true>(l1a);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return l1a & l1ad1 & u1;
    }

    [[gnu::always_inline]]
    static constexpr type south(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto l1u1 = data_t::template shift_up<1>(l1);
        return l1 & r1 & l1u1 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type west(const type &free_space_block) {
        const auto r1a = free_space_block & data_t::template shift_right<1>(free_space_block);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto r1au1 = data_t::template shift_up<1>(r1a);
        return r1a & d1 & r1au1;
    }
};

template<typename Data>
struct free_spaces<Data, Shape::I> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 4;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        const auto n = north(free_space_block);
        const auto e = east(free_space_block);
        return {n, e, n, e};
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto l2 = data_t::template shift_left<2>(free_space_block);
        return r1 & l1 & l2 & free_space_block;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        const auto u2 = data_t::template shift_up<2>(free_space_block);
        return d1 & u1 & u2 & free_space_block;
    }
};

template<typename Data>
struct free_spaces<Data, Shape::S> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 4;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        const auto n = north(free_space_block);
        const auto e = east(free_space_block);
        return {n, e, n, e};
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto l1d1a = free_space_block & data_t::template shift_down<1, true>(l1);
        const auto l1d1ar1 = free_space_block & data_t::template shift_right<1>(l1d1a);
        return l1d1a & l1d1ar1;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto d1a = free_space_block & data_t::template shift_down<1, true>(free_space_block);
        const auto d1al1 = data_t::template shift_left<1>(d1a);
        const auto d1al1u1 = data_t::template shift_up<1>(d1al1);
        return d1a & d1al1u1;
    }
};

template<typename Data>
struct free_spaces<Data, Shape::Z> {
    using data_t = data<Data>;
    using type = typename data_t::type;
    static constexpr size_t N = 4;

    [[gnu::always_inline]]
    static constexpr std::array<type, N> get(const type &free_space_block) {
        const auto n = north(free_space_block);
        const auto e = east(free_space_block);
        return {n, e, n, e};
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        const auto r1d1a = free_space_block & data_t::template shift_down<1, true>(r1);
        const auto l1d1al1 = free_space_block & data_t::template shift_left<1>(r1d1a);
        return r1d1a & l1d1al1;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto l1d1a = free_space_block & data_t::template shift_down<1, true>(l1);
        const auto l1d1au1 = data_t::template shift_up<1>(l1d1a);
        return l1d1a & l1d1au1;
    }
};

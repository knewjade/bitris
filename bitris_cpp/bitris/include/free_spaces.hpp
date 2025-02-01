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
        const auto d1a = free_space_block & data_t::template shift_down<1, true>(free_space_block);
        return d1a & data_t::template shift_left<1>(d1a);
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
        const auto d1a = free_space_block & data_t::template shift_down<1, true>(free_space_block);
        const auto d1al1 = data_t::template shift_left<1>(d1a);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        return free_space_block & d1al1 & r1;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
        const auto l1au1 = data_t::template shift_up<1>(l1a);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        return free_space_block & d1 & l1au1;
    }

    [[gnu::always_inline]]
    static constexpr type south(const type &free_space_block) {
        const auto u1a = free_space_block & data_t::template shift_up<1>(free_space_block);
        const auto u1ar1 = data_t::template shift_right<1>(u1a);
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        return free_space_block & u1ar1 & l1;
    }

    [[gnu::always_inline]]
    static constexpr type west(const type &free_space_block) {
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto d1r1 = data_t::template shift_right<1>(d1);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return free_space_block & d1 & u1 & d1r1;
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
        const auto d1a = free_space_block & data_t::template shift_down<1, true>(free_space_block);
        const auto d1ar1 = data_t::template shift_right<1>(d1a);
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        return free_space_block & d1ar1 & l1;
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto d1l1 = data_t::template shift_left<1>(d1);
        const auto u1 = data_t::template shift_up<1>(free_space_block);
        return free_space_block & d1 & u1 & d1l1;
    }

    [[gnu::always_inline]]
    static constexpr type south(const type &free_space_block) {
        const auto u1a = free_space_block & data_t::template shift_up<1>(free_space_block);
        const auto u1al1 = data_t::template shift_left<1>(u1a);
        const auto r1 = data_t::template shift_right<1>(free_space_block);
        return free_space_block & u1al1 & r1;
    }

    [[gnu::always_inline]]
    static constexpr type west(const type &free_space_block) {
        const auto r1a = free_space_block & data_t::template shift_right<1>(free_space_block);
        const auto r1au1 = data_t::template shift_up<1>(r1a);
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        return free_space_block & r1au1 & d1;
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
        const auto w = west(free_space_block);
        return {
                n,
                data_t::template shift_up<1>(w),
                data_t::template shift_right<1>(n),
                w,
        };
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto l2a = free_space_block & data_t::template shift_left<2>(free_space_block);
        return l2a & data_t::template shift_right<1>(l2a);
    }

    [[gnu::always_inline]]
    static constexpr type west(const type &free_space_block) {
        const auto d2a = free_space_block & data_t::template shift_down<2, true>(free_space_block);
        return d2a & data_t::template shift_up<1>(d2a);
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
        return {
                n,
                e,
                data_t::template shift_up<1>(n),
                data_t::template shift_right<1>(e),
        };
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto d1l1a = free_space_block & data_t::template shift_left<1>(d1);
        return d1l1a & data_t::template shift_right<1>(d1l1a);
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto d1a = free_space_block & data_t::template shift_down<1, true>(free_space_block);
        const auto d1al1 = data_t::template shift_left<1>(d1a);
        return d1a & data_t::template shift_up<1>(d1al1);
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
        return {
                n,
                e,
                data_t::template shift_up<1>(n),
                data_t::template shift_right<1>(e),
        };
    }

private:
    [[gnu::always_inline]]
    static constexpr type north(const type &free_space_block) {
        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
        const auto d1lra = free_space_block & data_t::template shift_right<1>(d1);
        return d1lra & data_t::template shift_left<1>(d1lra);
    }

    [[gnu::always_inline]]
    static constexpr type east(const type &free_space_block) {
        const auto l1 = data_t::template shift_left<1>(free_space_block);
        const auto l1d1a = free_space_block & data_t::template shift_down<1, true>(l1);
        return l1d1a & data_t::template shift_up<1>(l1d1a);
    }
};

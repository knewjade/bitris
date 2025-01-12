#pragma once

#include "pieces.hpp"
#include "data.hpp"



//
//template<typename, Shape>
//struct free_spaces {
//};
//
//template<typename D>
//struct free_spaces<D, Shape::O> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 1;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        return {north(free_space_block)};
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
//        return l1a & data_t::template shift_down<1, true>(l1a);
//    }
//};
//
//template<typename D>
//struct free_spaces<D, Shape::T> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 4;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        return {
//            north(free_space_block),
//            east(free_space_block),
//            sourth(free_space_block),
//            west(free_space_block),
//        };
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
//        return l1 & r1 & d1 & free_space_block;
//    }
//
//    static constexpr type east(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
//        const auto u1 = data_t::template shift_up<1>(free_space_block);
//        return l1 & d1 & u1 & free_space_block;
//    }
//
//    static constexpr type sourth(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto u1 = data_t::template shift_up<1>(free_space_block);
//        return l1 & r1 & u1 & free_space_block;
//    }
//
//    static constexpr type west(const type &free_space_block) {
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
//        const auto u1 = data_t::template shift_up<1>(free_space_block);
//        return r1 & d1 & u1 & free_space_block;
//    }
//};
//
//template<typename D>
//struct free_spaces<D, Shape::L> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 4;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        return {
//            north(free_space_block),
//            east(free_space_block),
//            sourth(free_space_block),
//            west(free_space_block),
//        };
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto l1d1 = data_t::template shift_down<1, true>(l1);
//        return l1 & r1 & l1d1 & free_space_block;
//    }
//
//    static constexpr type east(const type &free_space_block) {
//        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
//        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
//        const auto l1au1 = data_t::template shift_up<1>(l1a);
//        return l1a & d1 & l1au1;
//    }
//
//    static constexpr type sourth(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto r1u1 = data_t::template shift_up<1, true>(r1);
//        return l1 & r1 & r1u1 & free_space_block;
//    }
//
//    static constexpr type west(const type &free_space_block) {
//        const auto r1a = free_space_block & data_t::template shift_right<1>(free_space_block);
//        const auto r1ad1 = data_t::template shift_down<1, true>(r1a);
//        const auto u1 = data_t::template shift_up<1>(free_space_block);
//        return r1a & r1ad1 & u1;
//    }
//};
//
//template<typename D>
//struct free_spaces<D, Shape::J> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 4;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        return {
//            north(free_space_block),
//            east(free_space_block),
//            sourth(free_space_block),
//            west(free_space_block),
//        };
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto r1d1 = data_t::template shift_down<1, true>(r1);
//        return l1 & r1 & r1d1 & free_space_block;
//    }
//
//    static constexpr type east(const type &free_space_block) {
//        const auto l1a = free_space_block & data_t::template shift_left<1>(free_space_block);
//        const auto l1ad1 = data_t::template shift_down<1, true>(l1a);
//        const auto u1 = data_t::template shift_up<1>(free_space_block);
//        return l1a & l1ad1 & u1;
//    }
//
//    static constexpr type sourth(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto l1u1 = data_t::template shift_up<1, true>(l1);
//        return l1 & r1 & l1u1 & free_space_block;
//    }
//
//    static constexpr type west(const type &free_space_block) {
//        const auto r1a = free_space_block & data_t::template shift_right<1>(free_space_block);
//        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
//        const auto r1au1 = data_t::template shift_up<1>(r1a);
//        return r1a & d1 & r1au1;
//    }
//};
//
//template<typename D>
//struct free_spaces<D, Shape::I> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 4;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        const auto n = north(free_space_block);
//        const auto e = east(free_space_block);
//        return {n, e, n, e};
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto l2 = data_t::template shift_left<2>(free_space_block);
//        return r1 & l1 & l2 & free_space_block;
//    }
//
//    static constexpr type east(const type &free_space_block) {
//        const auto d1 = data_t::template shift_down<1, true>(free_space_block);
//        const auto u1 = data_t::template shift_up<1>(free_space_block);
//        const auto u2 = data_t::template shift_up<2>(free_space_block);
//        return d1 & u1 & u2 & free_space_block;
//    }
//};
//
//template<typename D>
//struct free_spaces<D, Shape::S> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 4;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        const auto n = north(free_space_block);
//        const auto e = east(free_space_block);
//        return {n, e, n, e};
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto l1d1a = free_space_block & data_t::template shift_down<1, true>(l1);
//        const auto l1d1ar1 = free_space_block & data_t::template shift_right<1>(l1d1a);
//        return l1d1a & l1d1ar1;
//    }
//
//    static constexpr type east(const type &free_space_block) {
//        const auto d1a = free_space_block & data_t::template shift_down<1, true>(free_space_block);
//        const auto d1al1 = data_t::template shift_left<1>(d1a);
//        const auto d1al1u1 = data_t::template shift_up<1>(d1al1);
//        return d1a & d1al1u1;
//    }
//};
//
//template<typename D>
//struct free_spaces<D, Shape::Z> {
//    using data_t = data<D>;
//    using type = typename data_t::type;
//    static constexpr size_t N = 4;
//
//    static constexpr std::array<type, N> get(const type &free_space_block) {
//        const auto n = north(free_space_block);
//        const auto e = east(free_space_block);
//        return {n, e, n, e};
//    }
//
//    static constexpr type north(const type &free_space_block) {
//        const auto r1 = data_t::template shift_right<1>(free_space_block);
//        const auto r1d1a = free_space_block & data_t::template shift_down<1, true>(r1);
//        const auto l1d1al1 = free_space_block & data_t::template shift_left<1>(r1d1a);
//        return r1d1a & l1d1al1;
//    }
//
//    static constexpr type east(const type &free_space_block) {
//        const auto l1 = data_t::template shift_left<1>(free_space_block);
//        const auto l1d1a = free_space_block & data_t::template shift_down<1, true>(l1);
//        const auto l1d1au1 = data_t::template shift_up<1>(l1d1a);
//        return l1d1a & l1d1au1;
//    }
//};

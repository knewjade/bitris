#ifndef TEMPLATES_HPP
#define TEMPLATES_HPP

#include <utility>

template<std::size_t iterations, typename F>
[[gnu::always_inline]]
constexpr void static_for(F &&function) {
    constexpr auto f = []<std::size_t... S>(F &&callable, std::index_sequence<S...>) {
        const auto unpack = {
            0,
            (void(callable(std::integral_constant<std::size_t, S>{})), 0)...
        };
        (void) unpack;
    };
    f(std::forward<F>(function), std::make_index_sequence<iterations>());
}

template<std::size_t Start, std::size_t End>
[[gnu::always_inline]]
constexpr auto make_offset_index_sequence() {
    static_assert(Start <= End);
    constexpr auto f = []<std::size_t... Indices>(std::index_sequence<Indices...>) {
        return std::index_sequence<(Start + Indices)...>{};
    };
    return f(std::make_index_sequence<End - Start>{});
}

template<std::size_t Start, std::size_t End, typename F>
[[gnu::always_inline]]
constexpr void static_for_range(F &&function2) {
    static_assert(Start <= End);
    constexpr auto f = []<std::size_t... S>(F &&function, std::index_sequence<S...>) {
        const auto unpack = {
            0,
            (void(function(std::integral_constant<std::size_t, S>{})), 0)...
        };
        (void) unpack;
    };
    f(std::forward<F>(function2), make_offset_index_sequence<Start, End>());
}

template<typename T, typename F, std::size_t... S>
[[gnu::always_inline]]
constexpr T static_fold(F &&function, T init, std::index_sequence<S...>) {
    ((init = function(std::forward<T>(init), std::integral_constant<std::size_t, S>{})), ...);
    return std::forward<T>(init);
}

template<typename T, typename F, std::size_t N>
[[gnu::always_inline]]
constexpr T static_fold(F &&function, T init, const std::array<T, N> &value) {
    constexpr auto f = []<std::size_t... S>(
        F &&callable, T acc, const std::array<T, N> &vs, std::index_sequence<S...>) {
        ((acc = callable(std::move(acc), vs[S])), ...);
        return acc;
    };
    return f(std::forward<F>(function), std::move(init), value, std::make_index_sequence<N>());
}

template<typename T, typename F, std::size_t... S>
constexpr auto static_packing_fold(F &&function, const T &value, std::index_sequence<S...>) {
    return function(value[S]...);
}

template<typename T, typename F, std::size_t N>
constexpr auto static_packing_fold(F &&function, const std::array<T, N> &value) {
    constexpr auto f = []<std::size_t... S>(
        F &&callable, const std::array<T, N> &vs, std::index_sequence<S...>) {
            return callable(vs[S]...);
        };
    return f(std::forward<F>(function), value, std::make_index_sequence<N>());;
}

template<typename T, typename F, std::size_t... S>
[[gnu::always_inline]]
constexpr T static_transform(F &&function, T value, std::index_sequence<S...>) {
    ((value[S] = function(value[S])), ...);
    return value;
}

template<typename T, typename F, std::size_t N>
[[gnu::always_inline]]
constexpr std::array<T, N> static_transform(F &&function, const std::array<T, N> &value) {
    auto buffer = std::array<T, N>{};
    constexpr auto f = []<std::size_t... S>(
        F &&callable, std::array<T, N> &bs, const std::array<T, N> &vs, std::index_sequence<S...>) {
        ((bs[S] = callable(vs[S])), ...);
    };
    f(std::forward<F>(function), buffer, value, std::make_index_sequence<N>());
    return buffer;
}

template<typename T, typename F, std::size_t N>
[[gnu::always_inline]]
constexpr std::array<T, N> static_transform_indexed(F &&function, const std::array<T, N> &value) {
    auto buffer = std::array<T, N>{};
    constexpr auto f = []<std::size_t... S>(
        F &&callable, std::array<T, N> &bs, const std::array<T, N> &vs, std::index_sequence<S...>) {
            ((bs[S] = callable(S, vs[S])), ...);
        };
    f(std::forward<F>(function), buffer, value, std::make_index_sequence<N>());
    return buffer;
}

template<typename T, typename F, std::size_t N>
[[gnu::always_inline]]
constexpr std::array<T, N> static_zip2(F &&function, const std::array<T, N> &arr1, const std::array<T, N> &arr2) {
    auto buffer = std::array<T, N>{};
    constexpr auto f = []<std::size_t... S>(
        F &&callable, std::array<T, N> &bs, const std::array<T, N> &vs1, const std::array<T, N> &vs2, std::index_sequence<S...>) {
            ((bs[S] = callable(vs1[S], vs2[S])), ...);
        };
    f(std::forward<F>(function), buffer, arr1, arr2, std::make_index_sequence<N>());
    return buffer;
}

#endif //TEMPLATES_HPP

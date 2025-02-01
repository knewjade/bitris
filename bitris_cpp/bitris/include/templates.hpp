#pragma once

template<std::size_t iterations, typename F>
[[gnu::always_inline]]
constexpr void static_for_t(F &&function) {
    constexpr auto f = []<std::size_t... S> [[gnu::always_inline]](F &&callable, std::index_sequence<S...>) {
        const auto unpack = {
                0,
                (void(callable.template operator()<std::integral_constant<std::size_t, S>{}>()), 0)...
        };
        (void) unpack;
    };
    f(std::forward<F>(function), std::make_index_sequence<iterations>());
}

template<std::array Arr, typename F>
[[gnu::always_inline]]
constexpr void static_for_t(F &&function) {
    constexpr auto N = std::tuple_size_v<std::remove_reference_t<decltype(Arr)> >;
    constexpr auto f = []<std::size_t... S>(
            F &&callable, std::index_sequence<S...>) {
        (void(callable.template operator()<Arr[S]>()), ...);
    };
    f(std::forward<F>(function), std::make_index_sequence<N>());
}

template<std::size_t Iterations, typename T, typename F>
[[gnu::always_inline]]
constexpr T static_fold_t(F &&function, T init) {
    constexpr auto f = []<std::size_t... S>(F &&function, T init, std::index_sequence<S...>) {
        ((init = function.template operator()<std::integral_constant<std::size_t, S>{}>(std::forward<T>(init))), ...);
        return std::forward<T>(init);
    };
    return f(std::forward<F>(function), std::move(init), std::make_index_sequence<Iterations>());
}

template<typename T, typename F, std::size_t N>
constexpr auto static_packing_fold(F &&function, const std::array<T, N> &value) {
    constexpr auto f = []<std::size_t... S> [[gnu::always_inline]](
            F &&callable, const std::array<T, N> &vs, std::index_sequence<S...>) {
        return callable(vs[S]...);
    };
    return f(std::forward<F>(function), value, std::make_index_sequence<N>());;
}

template<typename T, typename F, std::size_t N>
[[gnu::always_inline]]
constexpr std::array<T, N> static_zip2(F &&function, const std::array<T, N> &arr1, const std::array<T, N> &arr2) {
    auto buffer = std::array<T, N>{};
    constexpr auto f = []<std::size_t... S> [[gnu::always_inline]](
            F &&callable, std::array<T, N> &bs, const std::array<T, N> &vs1, const std::array<T, N> &vs2,
            std::index_sequence<S...>) {
        ((bs[S] = callable(vs1[S], vs2[S])), ...);
    };
    f(std::forward<F>(function), buffer, arr1, arr2, std::make_index_sequence<N>());
    return buffer;
}

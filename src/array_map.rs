
#[inline(always)]
pub fn map_indexed4<F, T, U>(data: [T; 4], mut f: F) -> [U; 4]
where
    F: FnMut(usize, T) -> U {
    let [a, b, c, d] = data;
    [
        f(0, a),
        f(1, b),
        f(2, c),
        f(3, d),
    ]
}

#[inline(always)]
pub fn zip2_map4<F, T1, T2, U>(first: [T1; 4], second: [T2; 4], mut f: F) -> [U; 4]
where
    F: FnMut(T1, T2) -> U {
    let [a1, b1, c1, d1] = first;
    let [a2, b2, c2, d2] = second;
    [
        f(a1, a2),
        f(b1, b2),
        f(c1, c2),
        f(d1, d2),
    ]
}

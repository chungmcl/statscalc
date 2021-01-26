use super::general_funcs;

pub fn permutations_n(n: f64) -> f64 {
    general_funcs::factorial(n)
}

pub fn permutations_nr(n: f64, r: f64) -> f64 {
    general_funcs::factorial(n) / general_funcs::factorial(n - r)
}

pub fn combinations(n: f64, r: f64) -> f64 {
    general_funcs::factorial(n) / (general_funcs::factorial(n - r) * general_funcs::factorial(r))
}
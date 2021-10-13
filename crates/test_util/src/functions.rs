pub fn assert_vec_equal<T: std::cmp::PartialEq + std::fmt::Debug>(
    vec1: &[T],
    vec2: &[T],
    message: &'static str,
) {
    let mut equal = vec1.len() == vec2.len();
    for e in vec1 {
        equal = vec2.contains(e) && equal;
    }

    if !equal {
        panic!("\n{}: \n{:?}\n{:?}\n", message, vec1, vec2);
    }
}

pub fn do_vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let matching = a.iter().zip(b).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

use std::collections::HashMap;

fn trib_impl(trib_n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(f) = cache.get(&trib_n) {
        return *f;
    }

    let result =
        trib_impl(trib_n - 1, cache) + trib_impl(trib_n - 2, cache) + trib_impl(trib_n - 3, cache);
    cache.insert(trib_n, result);
    result
}

fn trib(trib_n: usize) -> usize {
    let mut cache = HashMap::new();
    cache.insert(0, 0);
    cache.insert(1, 0);
    cache.insert(2, 1);
    trib_impl(trib_n, &mut cache)
}

fn main() {
    dp_school::read!(trib_n as usize);
    println!("{}", trib(trib_n + 2))
}

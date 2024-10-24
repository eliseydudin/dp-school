use std::collections::HashMap;

fn fib_impl(fib_n: usize, cache: &mut HashMap<usize, u16>) -> u16 {
    if let Some(f) = cache.get(&fib_n) {
        return *f;
    }

    let result = (fib_impl(fib_n - 1, cache) + fib_impl(fib_n - 2, cache)) % 10;
    cache.insert(fib_n, result);
    result
}

fn fib(fib_n: usize) -> u16 {
    let mut cache = HashMap::new();
    cache.insert(0, 1);
    cache.insert(1, 1);
    fib_impl(fib_n, &mut cache)
}

fn main() {
    dp_school::read!(fib_n as usize);
    println!("{}", fib(fib_n))
}

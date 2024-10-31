# 24/10 Динамическое программирование   
Динамическое программирование - способ решения сложных задач путём разбиения их на более простые подзадачи.   
Главный пример ДП - Числа Фибоначчи.
Например вам нужно посчитать пятое число данной последовательности, используя рекурсию это можно сделать так:   
```math
F5 = F4 + F3 = (F3 + F2 + F1) + (F2 + F1) = ...
```
Здесь мы считаем F2 несколько раз, в ДП мы добавляем хэш-таблицу, чтобы хранить уже посчитанные значения для избежания бесполезных вычислений.   
```rust
use std::collections::HashMap;

fn fib_impl(fib_n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(f) = cache.get(&fib_n) {
        return *f;
    }

    let result = fib_impl(fib_n - 1, cache) + fib_impl(fib_n - 2, cache);
    cache.insert(fib_n, result);
    result
}

fn fib(fib_n: usize) -> usize {
    let mut cache = HashMap::new();
    cache.insert(0, 1);
    cache.insert(1, 1);
    fib_impl(fib_n, &mut cache)
}

fn main() {
	/// Макрос [`read`] определён в файле `src/lib.rs`
    dp_school::read!(fib_n as usize);
    println!("{}", fib(fib_n))
}

```
# Задача 203: Мячик на лесенке   
> На вершине лесенки, содержащей N ступенек, находится мячик, который начинает прыгать по ним вниз, к основанию. Мячик может прыгнуть на следующую ступеньку, на ступеньку через одну или через 2. (То есть, если мячик лежит на 8-ой ступеньке, то он может переместиться на 5-ую, 6-ую или 7-ую.) Определить число всевозможных "маршрутов" мячика с вершины на землю.   

По факту эта задача заключается в нахождении N-ого числа трибоначчи.    
```rust
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

```
# Задача 842: Последняя цифра числа Фибоначчи   
Тоже самое, что и в первой, только число нужно делить по модулю на 10.   
```rust
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

```
# Задача 843: Простая последовательность   
Та же задача про числа Фибоначчи, только условия чуть-чуть изменены   
```rust
fn main() {
    dp_school::read!(n as usize);
    let mut vec: Vec<usize> = vec![0; n + 1];
    vec[0] = 1;
    vec[1] = 1;

    for i in 2..(n + 1) {
        vec[i] = if i % 2 == 0 {
            vec[i / 2] + vec[i / 2 - 1]
        } else {
            vec[i / 2] - vec[i / 2 - 1]
        }
    }

    println!("{}", vec[n])
}


```
# Задача 844: Какая-то последовательность (я забыл :skull:)   
```rust
fn main() {
    dp_school::read!(n as usize);
    let mut vec: Vec<usize> = vec![0; n + 1];
    vec[0] = 1;
    vec[1] = 1;

    for i in 2..=n {
        vec[i] = if i % 2 == 0 {
            vec[i / 2] + 1
        } else {
            vec[(i + 1) / 2] + 1 - vec[(i - 1) / 2]
        }
    }

    println!("{}", vec[n])
}

```

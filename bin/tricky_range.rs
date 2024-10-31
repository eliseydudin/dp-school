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

// Stolen from https://github.com/ognevny/ognlib/blob/master/src/macros.rs

#[macro_export]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = std::string::String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out: Vec<$type> = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect();
    };
    ($out:ident as String) => {
        let mut inner = std::string::String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out: Vec<std::string::String> = inner.trim().split_whitespace().collect();
    };
}

#[macro_export]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = std::string::String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner.trim().parse::<$type>().unwrap();
    };
}

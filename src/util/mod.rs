pub mod http;

pub fn fetch_lines(day: i32, session: &str) -> Vec<String> {
    let response = http::get(day, session);
    let splt = response.split("\n");
    let mut strings = Vec::new();
    for s in splt {
        if s != "" {
            strings.push(String::from(s));
        }
    }
    strings
}

pub fn modulo<T: std::clone::Clone + std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy>(val: T, modulo: T) -> T {
    return ((val % modulo) + modulo) % modulo
}

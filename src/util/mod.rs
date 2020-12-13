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

pub fn modulo(a: i32, b: i32) -> i32 {
    if b == 0 {
        return i32::min_value();
    }
    return ((a % b) + b) % b
}

pub fn modulo64(a: i128, b: i128) -> i128 {
    if b == 0 {
        return i128::min_value();
    }
    return ((a % b) + b) % b
}
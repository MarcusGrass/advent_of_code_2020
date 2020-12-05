pub mod http;

pub fn fetch_lines(day: i32, session: &str) -> Vec<String> {
    let response = http::get(day, session);
    let splt = response.split("\n");
    let mut strings = Vec::new();
    for s in splt {
        strings.push(String::from(s));
    }
    strings
}

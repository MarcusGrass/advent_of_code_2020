use reqwest::blocking::Client;
use reqwest::{Method, Url};
use reqwest::header::HeaderValue;

pub fn get(day: i32, session: &str) -> String {
    let url = format!("https://adventofcode.com/2020/day/{}/input", day);
    let client = Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let mut req = reqwest::blocking::Request::new(Method::GET, Url::parse(&url).unwrap());
    let header = HeaderValue::from_bytes(format!("session={}", session).as_bytes())
        .unwrap();
    req.headers_mut().insert("cookie", header);
    String::from_utf8(
        client.execute(req)
            .unwrap()
            .bytes()
            .unwrap()
            .to_vec()
    ).unwrap()

}

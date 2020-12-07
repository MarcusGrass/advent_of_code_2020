use std::collections::HashMap;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(4, session);

    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    println!("4.1 = {}", solve(lines, &contains_correct_first));
}

fn solve_second(lines: &Vec<String>) {
    println!("4.2 = {}", solve(lines, &contains_correct_second));
}

fn solve(lines: &Vec<String>, validator: &dyn Fn(&Vec<String>) -> i32) -> i32{
    let mut trimmed: Vec<Vec<String>> = Vec::new();
    let mut part = Vec::new();
    for line in lines {
        if line == "" {
            trimmed.push(part.clone());
            part = Vec::new();
        } else {
            for splt in line.split(" ") {
                part.push(String::from(splt));
            }
        }
    }
    let mut valid = 0;
    for fields in &trimmed {
        valid += validator(fields);
    }
    return valid;
}

fn contains_correct_first(fields: &Vec<String>) -> i32 {
    if fields.len() == 8 {
        return  1;
    } else if fields.len() == 7 {
        for field in fields {
            if field.starts_with("cid") {
                return 0;
            }
        }
        return 1;
    }
    return 0;
}

fn contains_correct_second(fields: &Vec<String>) -> i32 {
    let map = to_map(fields);
    if valid_year(map.get("byr"), 1920, 2002)
        && valid_year(map.get("iyr"), 2010, 2020)
        && valid_year(map.get("eyr"), 2020, 2030)
        && valid_height(map.get("hgt"))
        && valid_hair_color(map.get("hcl"))
        && valid_pid(map.get("pid"))
        && valid_eye_color(map.get("ecl")) {
        return 1;
    }
    return 0;
}

fn to_map(fields: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for field in fields {
        let mut splt = field.split(":");
        map.insert(String::from(splt.next().unwrap()), String::from(splt.next().unwrap()));
    }
    map
}

fn valid_year(year: Option<&String>, min: i32, max: i32) -> bool {
    year.and_then(|s| s.parse().ok())
        .map(|y: i32| y >= min && y <= max)
        .unwrap_or(false)
}

fn valid_height(height: Option<&String>) -> bool {
    height.and_then(|s| -> Option<(i32, &str)>{
        let mut it = 0;
        for chr in s.chars() {
            if !chr.is_numeric() {
                break;
            }
            it += 1;
        }
        let splt = s.split_at(it);
        return splt.0.parse().ok().zip(Some(splt.1));
    })
        .map(|t| {
            return if t.1 == "cm" {
                t.0 >= 150 && t.0 <= 193
            } else if t.1 == "in" {
                t.0 >= 59 && t.0 <= 76
            } else {
                false
            }
        })
        .unwrap_or(false)
}

fn valid_eye_color(eye: Option<&String>) -> bool {
    eye.map(|h| {
        match h.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        }
    })
        .unwrap_or(false)
}

fn valid_hair_color(hair: Option<&String>) -> bool {
    hair.map(|h| {
        if h.len() != 7 {
            return false;
        }
        let mut it = 0;
        for chr in h.chars() {
            let as_str = chr.to_string();
            if it == 0 {
                if as_str != "#" {
                    return false;
                }
            } else {
                if chr.is_numeric() {
                    continue;
                } else if chr.is_alphabetic(){
                    if chr.le(&'f') {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
            it += 1;
        }
        return true;
    })
        .unwrap_or(false)
}

fn valid_pid(pid: Option<&String>) -> bool {
    pid.map(|p| {
        if p.len() != 9 {
            return false;
        }
        for chr in p.chars() {
            if !chr.is_numeric() {
                return false;
            }
        }
        return true;
    })
        .unwrap_or(false)
}


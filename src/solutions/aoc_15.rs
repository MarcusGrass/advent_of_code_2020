use std::collections::HashMap;

pub fn solve_both() {
    let numbers = vec![7, 14, 0, 17, 11, 1, 2];
    solve_first(&numbers);
    solve_second(&numbers);

}

fn solve_first(numbers: &Vec<usize>) {
    println!("15.1 = {:?}", solve(numbers, 2020));
}

fn solve_second(numbers: &Vec<usize>) {
    println!("15.2 = {:?}", solve(numbers, 30000000));
}

fn solve(numbers: &Vec<usize>, it: usize) -> usize {
    let mut map = HashMap::new();
    let mut last = Spoken{last_spoken: 0, num: 0};
    for i in 1..it + 1 {
        if i < numbers.len() + 1 {
            map.insert(numbers[i - 1], i);
            last = Spoken{last_spoken: 0, num: numbers[i - 1]}
        } else {
            let num;
            if last.last_spoken == 0 {
                num = 0;
                map.insert(last.num, i - 1);
            } else {
                num = match map.get(&last.num) {
                    Some(u) => i - u,
                    None => 0
                } - 1;
                map.insert(last.num, i - 1);
            }
            let times = if map.contains_key(&num) {*map.get(&num).unwrap()} else {0 as usize};
            last = Spoken{num, last_spoken: times};
        }
    }
    last.num
}

#[derive(Debug, Copy, Clone)]
struct Spoken {
    last_spoken: usize,
    num: usize,
}

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(9, session);
    let numbers = to_numbers(&lines);

    let needle = solve_first(&numbers);
    solve_second(needle, &numbers);
}

fn solve_first(numbers: &Vec<i64>) -> i64 {
    let mut target = 0;
    for i in 25..numbers.len() {
        let mut found = false;
        for j in (i-25)..i {
            for k in (i - 25)..i {
                if j == k {
                    continue;
                }
                if numbers[j] + numbers[k] == numbers[i] {
                    found = true;
                }
            }
        }
        if found != true {
            target = numbers[i];
            break;
        }
    }
    println!("5.1 = {}", target);
    target
}

fn solve_second(needle: i64, numbers: &Vec<i64>) {
    let mut start = 0;
    loop {
        let mut sum = 0;
        let mut nums: Vec<i64> = Vec::new();
        for i in start..numbers.len() {
            nums.push(numbers[i]);
            sum += numbers[i];
            if sum == needle {
                println!("5.2 = {}", nums.iter().min().unwrap() + nums.iter().max().unwrap());
                return;
            } else if sum > needle {
                break;
            }
        }
        start += 1;
    }
}

fn to_numbers(lines: &Vec<String>) -> Vec<i64> {
    let mut numbers = Vec::with_capacity(lines.len());
    for line in lines {
        numbers.push(line.parse().unwrap())
    }
    numbers
}

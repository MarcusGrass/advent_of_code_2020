use crate::solutions::aoc_18::Operator::{PLUS, MULT};

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(18, session);
    solve_first(&lines);
    solve_second(&lines);
}


fn solve_first(lines: &Vec<String>) {
    let mut sum = 0u128;
    for line in lines {
        let val = to_cluster(line).evaluate_straight();
        sum += val;
    }
    println!("18.1 = {:?}", sum);
}

fn do_op(a: u128, b: u128, op: Operator) -> u128 {
    match op {
        PLUS => a + b,
        MULT => a * b
    }
}

fn solve_second(lines: &Vec<String>) {
    let mut sum = 0u128;
    for line in lines {
        let val = to_cluster(line).evaluate_sum_prio();
        sum += val;
    }
    println!("18.2 = {:?}", sum);
}

fn to_cluster(line: &String) -> Cluster {
    let mut nested = Vec::new();
    nested.push(Cluster::parent());
    for char in line.chars() {
        let len = &nested.len() - 1;
        if char.is_numeric() {
            let num = char.to_digit(10).unwrap() as u128;
            nested.get_mut(len).unwrap().clusters.push(Cluster::child(num));
        } else if char == '+' {
            nested.get_mut(len).unwrap().operators.push(PLUS);
        } else if char == '*' {
            nested.get_mut(len).unwrap().operators.push(MULT);
        } else if char == '(' {
            nested.push(Cluster::parent());
        } else if char == ')' {
            let len = &nested.len() - 1;
            let last_nested = nested.remove(len);
            nested.get_mut(len - 1).unwrap().clusters.push(last_nested);
        }
    }
    nested.remove(0)
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Cluster {
    clusters: Vec<Cluster>,
    operators: Vec<Operator>,
    val: Option<u128>
}

impl Cluster {
    fn child(val: u128) -> Cluster {
        Cluster{val: Some(val), clusters: Vec::new(), operators: Vec::new()}
    }
    fn parent() -> Cluster {
        Cluster{val: None, clusters: Vec::new(), operators: Vec::new()}
    }

    fn evaluate_straight(&self) -> u128 {
        if self.val.is_some() {
            self.val.unwrap()
        } else {
            let mut sum = do_op(self.clusters[0].evaluate_straight(), self.clusters[1].evaluate_straight(), self.operators[0]);
            for i in 2..self.clusters.len() {
                sum = do_op(sum, self.clusters[i].evaluate_straight(), self.operators[i - 1]);
            }
            sum
        }
    }

    fn insert_parens(&self) -> Cluster {
        if self.val.is_some() {
            return self.clone();
        }
        let mut vals: Vec<Cluster> = Vec::new();
        let mut i = 0;
        vals.push(self.clusters[i].clone().insert_parens());
        while i < self.clusters.len() - 1 {
            if self.operators[i] == PLUS {
                let prev = vals.remove(vals.len() - 1);
                vals.push(Cluster{val: None, clusters: vec![prev.insert_parens(), self.clusters[i + 1].insert_parens()], operators: vec![PLUS]});
            } else {
                vals.push(self.clusters[i + 1].clone().insert_parens());
            }
            i += 1
        }
        if vals.len() == 1 {
            return vals[0].clone();
        }

        let mut ops = Vec::new();
        for _i in 0..vals.len() - 1 {
            ops.push(MULT);
        }
        Cluster{val: None, clusters: vals, operators: ops}
    }

    fn evaluate_sum_prio(&self) -> u128 {
        self.insert_parens().evaluate_straight()
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Operator {
    PLUS, MULT
}
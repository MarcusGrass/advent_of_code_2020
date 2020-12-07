use std::collections::{HashSet, HashMap};
use std::borrow::BorrowMut;

pub fn solve_both(session: &str) {
    let lines = crate::util::fetch_lines(7, session);
    solve_first(&lines);
    solve_second(&lines);
}

fn solve_first(lines: &Vec<String>) {
    let mut bags = HashMap::new();
    for line in lines {
        let bag = to_bag(line);
        bags.insert(String::from(&bag.color), bag);
    }
    println!("7.1 = {}", search_for("shiny gold", bags));
}

fn solve_second(lines: &Vec<String>) {
    let mut bags = HashMap::new();
    for line in lines {
        let bag = to_bag(line);
        bags.insert(String::from(&bag.color), bag);
    }
    println!("7.2 = {}", search_contained("shiny gold", bags));
}

fn to_bag(line: &String) -> Bag {
    let splt: Vec<&str> = line.split(" ").collect();
    let color = format!("{} {}", splt[0], splt[1]);
    let mut bags = HashMap::new();
    let mut it = 4;
    let mut num = 0;
    let mut sub_color: String = String::new();
    while it < splt.len() {
        if splt[it].len() == 1 {
            num = splt[it].parse().unwrap();
        } else {
            if splt[it].contains("bag") {
                bags.insert(sub_color.clone(), Content{
                    bag: Bag{
                        color: sub_color.clone(),
                        contents: HashMap::new(),
                    },
                    amount: num
                });
                sub_color = String::new();
            } else if splt[it] == "no" && splt[it + 1] == "other" {
                return Bag{ color, contents: HashMap::new() }
            } else {
                if sub_color == "" {
                    sub_color = String::from(splt[it]);
                } else {
                    sub_color = format!("{} {}", sub_color, splt[it]);
                }
            }
        }
        it += 1;
    }
    Bag {
        color,
        contents: bags
    }
}

fn search_for(color: &str, bags: HashMap<String, Bag>) -> i32 {
    let root = construct_reverse_bag_tree(color, bags);
    find_unique_containing(&root).len() as i32
}

fn search_contained(color: &str, bags: HashMap<String, Bag>) -> i32 {
    let root = bags.get(color).unwrap();
    recurse_add(root, &bags)
}

fn recurse_add(root: &Bag, bags: &HashMap<String, Bag>) -> i32 {
    let mut hits = 0;
    for bag in &root.contents {
        let multiplier = bag.1.amount;
        hits += bag.1.amount;
        hits += recurse_add(bags.get(&bag.1.bag.color).unwrap(),  bags) * multiplier;
    }
    hits
}

fn construct_reverse_bag_tree(color: &str, bags: HashMap<String, Bag>) -> BagTree {
    let mut root = BagTree{
        bag: color.to_string(),
        nodes: vec![]
    };
    let mut trees = Vec::new();
    trees.push(root.borrow_mut());
    loop {
        let mut change = false;
        for tree in trees.iter_mut() {
            let curr = find_containing(&tree, &bags);
            if !curr.is_empty() {
                change = true;
            }
            tree.nodes = curr;
        }
        if !change {
            break;
        }
        let mut next: Vec<&mut BagTree> = Vec::new();
        for tree in trees {
            for node in tree.nodes.iter_mut() {
                next.push(node);
            }
        }
        trees = next;
    }
    root
}

fn find_containing(target: &BagTree, bags: &HashMap<String, Bag>) -> Vec<BagTree> {
    let mut tree = Vec::new();
    for bag in bags {
        if bag.0 == &target.bag {
            continue;
        }
        if bag.1.contents.contains_key(&target.bag) {
            tree.push(BagTree{
                bag: bag.1.color.to_string(),
                nodes: vec![]
            });
        }
    }
    tree
}

fn find_unique_containing(root: &BagTree) -> HashSet<String> {
    let mut containing = HashSet::new();
    for bags in &root.nodes {
        containing.insert(String::from(&bags.bag));
        containing.extend(find_unique_containing(&bags));
    }
    containing
}

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contents: HashMap<String, Content>,
}

#[derive(Debug, Clone)]
struct Content {
    bag: Bag,
    amount: i32,
}

#[derive(Debug, Clone)]
struct BagTree {
    bag: String,
    nodes: Vec<BagTree>,
}


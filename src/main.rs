extern crate rand;
use rand::distributions::{Distribution, Uniform};

fn branch(n: i32, i: i32) -> String {
    let mut result = String::from("");
    for _ in 0..(n - i) {
        result.push_str(" ");
    }
    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..10);
    for _ in 0..(2 * i + 1) {
        let r: i32 = between.sample(&mut rng);
        let tile = if r < 3 {
            "O"
        } else {
            "x"
        };
        result.push_str(tile);
    }
    result
}

fn star(n: i32) -> String {
    let mut result = String::from("");
    for _ in 0..n {
        result.push_str(" ");
    }
    result.push_str("+");
    result
}

fn trunk(n : i32) -> String {
    let mut result = String::from("");
    for _ in 0..(n - 1) {
        result.push_str(" ");
    }
    result.push_str("|||");
    result
}

fn get_tree(n: i32) -> Vec<String> {
    let mut output = Vec::new();
    output.push(star(n));
    for i in 0..n {
        output.push(branch(n, i));
    }
    output.push(trunk(n));
    output
}

fn main() {
    let output = get_tree(8);
    for l in output {
        println!("{}", l)
    }
}
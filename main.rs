fn get_tree(n: i32) -> Vec<String> {
    let mut output = Vec::new();
    output.push(String::from(" + "));
    output.push(String::from("|||"));
    output
}

fn main() {
    let output = get_tree(8);
    for l in output {
        println!("{}", l)
    }
}
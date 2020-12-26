fn get_tree(n: i32) -> String {
    let mut output = String::from(" + \n");
    output.push_str("|||");
    output
}

fn main() {
    let output = get_tree(8);
    println!("{}", output)
}
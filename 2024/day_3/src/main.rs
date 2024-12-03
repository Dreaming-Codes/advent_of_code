use regex::Regex;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let regex = Regex::new(r"(?P<do>do\(\))|(?P<dont>don't\(\))|(?P<mul>mul\((?P<num1>[0-9]{1,3}),(?P<num2>[0-9]{1,3})\))").unwrap();

    let matches = regex.captures_iter(&input);

    let mut result = 0;
    let mut operation_enabled = true;
    for op in matches {
        if op.name("do").is_some() {
            operation_enabled = true;
        } else if op.name("dont").is_some() {
            operation_enabled = false;
        } else if operation_enabled && op.name("mul").is_some() {
            let num1 = op.name("num1").unwrap().as_str().parse::<i32>().unwrap();
            let num2 = op.name("num2").unwrap().as_str().parse::<i32>().unwrap();
            result += num1 * num2;
        }
    }

    println!("Result: {}", result);
}

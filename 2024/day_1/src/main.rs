use std::collections::HashMap;
use std::io::Read;

fn part1(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(x, y)| i32::abs(x - y))
        .sum()
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let list2count: HashMap<i32, i32> = list2.iter().fold(HashMap::new(), |mut acc, &i| {
        *acc.entry(i).or_insert(0) += 1;
        acc
    });

    list1
        .iter()
        .map(|x| x * *list2count.get(x).unwrap_or(&0))
        .sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (list1, list2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split("   ");
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let part2result = part2(&list1, &list2);
    let part1result = part1(list1, list2);

    println!("{} {}", part1result, part2result);
}

fn main() {
    let input = include_str!("../input.txt");

    let f: u32 = input
        .lines()
        .map(|line| {
            let digits = line.chars().filter(|&c| c.is_numeric()).collect::<Vec<_>>();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            let c = format!("{}{}", first, last).parse::<u32>().unwrap();
            c
        })
        .sum();

    println!("{}", f)
}

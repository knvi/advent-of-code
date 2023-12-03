fn main() {
    let input = include_str!("../input.txt");

    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    let mut answer = 0;

    for line in input.lines() {
        let mut first = None;
        let mut last = 0;

        let mut to_digit = |d| {
            first = first.or(Some(d));
            last = d;
        };

        let chars = line.as_bytes();
        for (i, character) in chars.iter().enumerate() {
            if character.is_ascii_digit() {
                to_digit((character - b'0') as u32); // b'0' is the byte literal for '0' used to convert ascii to digit
            } else {
                for(f, digit_word) in digit_words.iter().enumerate() {
                    if chars[i..].starts_with(digit_word.as_bytes()) {
                        to_digit(f as u32 + 1)
                    }
                }
            }
        }

        answer += format!("{}{}", first.unwrap(), last).parse::<u32>().unwrap();
    }

    println!("{}", answer)
}

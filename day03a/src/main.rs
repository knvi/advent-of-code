use hashbrown::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    // parsing
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let mut symbols = HashMap::new();

    for (r, l) in lines.iter().enumerate() {
        let mut ch = 0;

        while ch < l.len() {
            let (start, mut symbol) = (ch, None);

            while ch < l.len() && l[ch].is_ascii_digit() { // if we got a number
                for (dr, dc) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] { // check all surroundings
                    let (rr, cc) = ((r as i32 + dr) as usize, (ch as i32 + dc) as usize); // get the coordinates of the surrounding
                    let Some(&s) = lines.get(rr).and_then(|l| l.get(cc)) else {continue}; // if we are out of bounds, continue
                    if s != b'.' && !s.is_ascii_digit() { // if we got a symbol
                        symbol = Some((cc, rr, s as char));
                        break; 
                    }
                } 
                ch += 1;
            }

            if start < ch {
                if let Some(symbol) = symbol {
                    let num = std::str::from_utf8(&l[start..ch]).unwrap().parse().unwrap();
                    symbols.entry(symbol).or_insert(Vec::new()).push(num);
                }
            }

            ch += 1;
        }
    }

    let answer: usize = symbols.values().flat_map(|v| v).sum();

    println!("{}", answer);
}

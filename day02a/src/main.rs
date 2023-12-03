#[derive(Debug, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let f: usize = input
        .lines()
        .map(|line| {
            let cube_data = line.split_once(":").unwrap().1;

            let mut cube_sets = Vec::new();
            for game in cube_data.split(";") {
                let mut cubes = Cubes::default();
                for i in game.split(",") {
                    let mut iter = i.trim().split_whitespace();
                    let amount = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();

                    match color {
                        "red" => cubes.red += amount,
                        "green" => cubes.green += amount,
                        "blue" => cubes.blue += amount,
                        _ => unreachable!(),
                    }
                }

                cube_sets.push(cubes);
            }
            cube_sets
        })
        .collect::<Vec<Vec<Cubes>>>()
        .iter()
        .enumerate()
        .filter(|(_, games)| games.iter().all(|game| game.is_possible()))
        .map(|x| x.0 + 1)
        .sum::<usize>()
        .into();

    println!("{}", f)
}

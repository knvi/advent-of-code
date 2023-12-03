#[derive(Debug, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn max(&self, s: &Self) -> Self {
        Self {
            red: self.red.max(s.red),
            green: self.green.max(s.green),
            blue: self.blue.max(s.blue),
        }
    }

    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let f: u32 = input
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
        .map(|x| {
            let mut max = Cubes::default();
            for game in x {
                max = max.max(game);
            }
            max.red * max.green * max.blue
        })
        .sum::<u32>()
        .into();

    println!("{}", f)
}

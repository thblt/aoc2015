const INPUT: u64 = 29_000_000;

fn part1() {
    for house in 1.. {
        let mut presents = 0;
        for elf in 1..=(house as f64).sqrt().floor() as u64 {
            if house % elf == 0 {
                presents += elf * 10;
                if house/elf != elf {
                    presents += (house/elf) * 10;
                }
            }
        }

        if presents >= INPUT {
            println!("Part 1: {house}");
            break
        }
    }
}

fn part2() {
    for house in 1.. {
        let mut presents = 0;
        for elf in 1..=(house as f64).sqrt().floor() as u64 {
            if house % elf == 0 {
                // println!("Found dividers {elf} and {}", house/elf);
                if house / elf <= 50
                    {
                        presents += elf * 11;
                    }
                if house/elf != elf {
                    let elf = house/elf;
                    if house / elf <= 50 {
                        presents += elf * 11;
                    }
                }
            }
        }
        if presents >= INPUT {
            println!("Part 2: {house}");
            break
        }
    }
}


fn main() {
    part1();
    part2();
}

use std::collections::HashMap;

type Aunt = HashMap<String, u64>;

fn main() {
    let mut mfcsam = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<Aunt>();

    'part1: for (number, sue) in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap().1)
        .enumerate()
    {
        let number = number + 1;
        let mut part1 = true;
        let mut part2 = true;
        for object in sue.split(", ") {
            let (name, count) = object.split_once(": ").unwrap();
            let count = count.parse::<u64>().unwrap();
            part1 &= mfcsam[name] == count;
            part2 &= if name == "cats" || name == "trees" {
                mfcsam[name] < count
            } else if name == "pomeranians" || name == "goldfish" {
                mfcsam[name] > count
            } else {
                mfcsam[name] == count
            };
        }
        if part1 {
            println!("Part 1: {number}");
        }
        if part2 {
            println!("Part 2: {number}");
        }
    }
}

fn part2(raw: &str) -> usize {
    let mut diff = 2; // New outer quotes
    for c in raw.chars() {
        match c {
            '\\' => diff += 1,
            '\"' => diff += 1,
            _ => {}
        }
    }
    diff
}

fn part1(raw: &str) -> usize {
    let mut skip = 0;
    let mut escape = false;
    let mut in_string = false;
    let mut done = false;
    let mut diff = 0;
    for c in raw.chars() {
        assert!(!done);
        if skip > 0 {
            skip -= 1;
            continue
        }
        diff += 1;
        match c {
            '"' if !in_string => in_string = true,
            '"' if in_string && !escape => done = true,
            '\\' if in_string && !escape => escape = true,
            'x' if in_string && escape => {escape = false; skip=2; diff+=1},
            _ if in_string => {diff -= 1; escape=false},
            _ => {  panic!("Abnormal state")}

        }
    };
    diff
}

fn main() {
    let mut result1 = 0;
    let mut result2 = 0;
    for line in std::fs::read_to_string("input").unwrap().split('\n').filter(|l| !l.is_empty()) {
        result1 +=  part1(line);
        result2 +=  part2(line);
    }
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}

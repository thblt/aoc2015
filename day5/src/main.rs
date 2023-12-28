fn is_nice(cand: &str) -> bool {
    if ["ab", "cd", "pq", "xy"].iter().all(|f| cand.find(f).is_none()) && cand.chars().filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u').count() >= 3 {
        let mut prev = '-';
        for c in cand.chars() {
            if c == prev {
                return true;
            } else {
                prev = c
            }
        }
    }
    false
}

fn is_nice2(cand: &str) -> bool {
    let mut test1 = false;
    let mut test2 = false;
    let mut pos1 = (0,0);
    let mut pos2 = 0;
    for i in 0..cand.len()
        {
            if i <= cand.len()-2 {
                let search = cand[i..i+2].to_string();
                if let Some(pos2) = cand[i+2..].find(&search) {
                    pos1 = (i, i+pos2+1);
                    test1 = true;
                }
            }
            if i >= 2 && cand.chars().nth(i-2) == cand.chars().nth(i) {
                test2 = true;
                pos2 = i;
            }
        }

    if test1 {
        for x in 0..=pos1.1 {
            if x == pos1.0 || x == pos1.1 {
                print!("v");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("{cand} is: {}",test1 && test2);
    if test2 {
        for x in 0..=pos2 {
            if x == pos2 - 2 || x == pos2 {
                print!("^");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    test1 && test2
}

fn main() {
    println!("Part 1: {}", std::fs::read_to_string("input").unwrap().split('\n').filter(|s| is_nice(s)).count());
    println!("Part 2: {}", std::fs::read_to_string("input").unwrap().split('\n').filter(|s| is_nice2(s)).count());
    println!("Test: {}", is_nice2("aaa"));
}

use std::collections::HashSet;

fn main() {
    // Read input
    let mut replacements = vec!();
    let mut template = String::new();
    let mut part1 = true;
    let binding = std::fs::read_to_string("input").unwrap();
    for line in binding.split('\n') {
        if line.is_empty() {
            part1 = false;
        } else if part1 {
            let (search, replace) = line.split_once(" => ").unwrap();
            replacements.push((search, replace));
        } else {
            template = line.to_string();
            break;
        }
    }

    // Part 1
    let mut outcomes = HashSet::new();
    for (search, replace) in replacements {
        let count = template.matches(search).count();
        let tpl = template.replace(search, "@");
        for i in 0..count {
            outcomes.insert(tpl.replacen("@", search, i)
                .replacen("@", replace, 1)
                .replace("@", search));
        }
    }
    println!("Part 1: {}", outcomes.len())

}

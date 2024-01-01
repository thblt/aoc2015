use std::collections::HashMap;

/// Enumerate all possible distributions, and store in result the
/// number of possible distributions per number of containers.
fn solve(containers: &[u64], solutions: &mut HashMap<u64, u64>, liters: u64, counter: u64) {
    if liters == 0 {
        return *solutions.entry(counter).or_insert(0) += 1;
    } else if containers.is_empty() {
        return;
    }
    let this_container = *containers.first().unwrap();
    if this_container <= liters {
        // Take this container
        solve(
            &containers[1..],
            solutions,
            liters - this_container,
            counter + 1
        )
    }
    // Skip this container
    solve(&containers[1..], solutions, liters, counter);
}

fn main() {
    let mut input = std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    input.sort_by(|a, b| b.cmp(a));

    let mut result = HashMap::new();
    solve(&input, &mut result, 150, 0);
    println!("Part 1: {}", result.values().sum::<u64>());
    println!("Part 2: {}", result[result.keys().min().unwrap()]);
}

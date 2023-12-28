struct Reeinder {
    speed: u64,
    run_time: u64,
    rest_time: u64,
    points: u64,
}

/// Compute the distance travelled by a reeinder in a given number of
/// seconds.
fn travelled_distance(
    race_time: u64,
    Reeinder {
        speed,
        run_time,
        rest_time,
        points: _,
    }: &Reeinder,
) -> u64 {
    // Reeinders run in cycles run + rest.  The total distance
    // travelled in race_time seconds is the number of cycles in that
    // time, times the distance travelled in a cycle: run_time *
    // speed.
    let cycle = run_time + rest_time;
    let mut dist = (race_time / cycle) * speed * run_time;
    // There may be a partial cycle.
    let extra_run_time = std::cmp::min(*run_time, race_time % cycle);
    dist += extra_run_time * speed;
    dist
}

/// Race reeinders for race_time seconds, award points at each second
/// (part 2), return the max total distance travelled by a reeinder at
/// the last second (part 1)
fn race(reeinders: &mut Vec<Reeinder>, race_time: u64) -> u64 {
    let mut best = 0;
    for time in 1..=race_time {
        let dists = reeinders
            .iter()
            .map(|r| travelled_distance(time, r))
            .collect::<Vec<u64>>();
        best = *dists.iter().max().unwrap();
        for rx in 0..reeinders.len() {
            if dists[rx] == best {
                reeinders[rx].points += 1;
            }
        }
    }
    best
}

fn main() {
    let mut reeinders = vec![];
    for line in std::fs::read_to_string("input").unwrap().split('\n') {
        if let Ok((_, speed, run_time, rest_time)) = sscanf::sscanf!(
            line,
            "{String} can fly {u64} km/s for {u64} seconds, but then must rest for {u64} seconds."
        ) {
            reeinders.push(Reeinder {
                speed,
                run_time,
                rest_time,
                points: 0,
            })
        }
    }
    let result1 = race(&mut reeinders, 2503);
    println!("Part 1: {result1}");
    println!(
        "Part 2: {}",
        reeinders.iter().map(|r| r.points).max().unwrap()
    );
}

use sscanf::sscanf;

fn main() {
    let mut lights: Vec<Vec<bool>> = vec!();
    for _ in 0..1000 {
        lights.push([false;1000].to_vec());
}
    for (instr, x1, y1, x2, y2) in std::fs::read_to_string("input").unwrap().split('\n').filter(|l| !l.is_empty()).map(|line| sscanf!(line, "{String} {usize},{usize} through {usize},{usize}").unwrap()) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                match &instr[0..] {
                    "toggle" => lights[y][x] = !lights[y][x],
                    "turn on" => lights[y][x] = true,
                    "turn off" => lights[y][x] = false,
                    _ => panic!(),
                }
            }
        }
    }
    println!("Part 1: {}",lights.iter().map(|line| line.iter().filter(|x| **x).count()).sum::<usize>());

    let mut lights: Vec<Vec<usize>> = vec!();
    for _ in 0..1000 {
        lights.push([0;1000].to_vec());
    }
    for (instr, x1, y1, x2, y2) in std::fs::read_to_string("input").unwrap().split('\n').filter(|l| !l.is_empty()).map(|line| sscanf!(line, "{String} {usize},{usize} through {usize},{usize}").unwrap()) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                match &instr[0..] {
                    "toggle" => lights[y][x] += 2,
                    "turn on" => lights[y][x] += 1,
                    "turn off" if lights[y][x] > 0 => lights[y][x] -= 1,
                    _ => {},
            }
            }
        }
    }
println!("Part 2: {}",lights.iter().map(|line| line.iter().sum::<usize>()).sum::<usize>());

}

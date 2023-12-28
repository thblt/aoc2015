const INPUT: &str = "1113222113";

fn play(input: &str) -> String {
    let mut ret = String::new();
    let mut char = None;
    let mut count = 1;
    for this in input.chars().chain(std::iter::once('X')) {
        if let Some(prev) = char {
            if prev != this {
                ret += count.to_string().as_str();
                ret.push(prev);
                char = Some(this);
                count = 1;
            } else {
                count += 1;
            }
        } else {
            char = Some(this)
        }
    }
    ret
    }

fn main() {
    let mut game = String::from(INPUT);
    // game = "1".to_string();
    for c in 0..50 {
        game = play(&game);
        if c == 40
            {println!("Part 1: {}", game.len());}
            }
    println!("Part 2: {}", game.len());
}

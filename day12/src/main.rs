fn main() {
    println!("Part 1: {}", std::fs::read_to_string("input").unwrap().split(|c: char| c != '-' && !c.is_ascii_digit()).filter(|c| !c.is_empty()).map(|cand| cand.parse::<i64>().unwrap()).sum::<i64>()) ;


}

use serde_json::Value;


fn count(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => if let Some(n) = n.as_i64() {
            n
        } else if let Some(n) = n.as_u64() {
            n as i64
        } else {
            unreachable!()
        },
        Value::String(_) => 0,
        Value::Array(arr) => arr.iter().map(|elem| count(&elem)).sum::<i64>(),
        Value::Object(obj) => {
            if obj.iter().any(|(_, v)| v == "red") {
                0
            } else {
                obj.iter().map(|(_, v)| count(&v)).sum::<i64>()
            }
        }
    }
}
fn main() {
    // Part 1 totally ignores JSON, and just splits the input at every
    // character that can't be part of a number, and sums the numbers
    // it finds.
    println!(
        "Part 1: {}",
        std::fs::read_to_string("input")
        .unwrap()
        .split(|c: char| c != '-' && !c.is_ascii_digit())
        .filter(|c| !c.is_empty())
        .map(|cand| cand.parse::<i64>().unwrap())
        .sum::<i64>()
    );

    // Alas!
    let v: Value = serde_json::from_str(&std::fs::read_to_string("input").unwrap()).unwrap();
    println!("Part 2: {}", count(&v));
}

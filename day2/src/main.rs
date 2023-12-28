trait AsTriple<T> {
    fn as_triple(self) -> (T,T,T);
}

impl<T: Copy> AsTriple<T> for Vec<T> {
    fn as_triple(self) -> (T,T,T) {
        (self[0], self[1], self[2])
    }
}

fn main() {
    let mut result1 = 0;
    let mut result2 = 0;
    for (length, width, height) in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .as_triple()
        })
    {
        let sides = [length * width, width * height, height * length];
        result1 += 2 * sides.iter().sum::<u32>() + sides.iter().min().unwrap();

        result2 +=
            // Ribbon
            (2 * [length + width, width + height, height + length].iter().min().unwrap())
                // Bow
                + (length * width * height);
    }
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}

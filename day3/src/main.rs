use std::collections::HashSet;

fn main() {
    let mut turn = false;
    let mut houses = HashSet::new();
    let mut houses2 = HashSet::new();
    let (mut x, mut y) = (0,0);
    let (mut x_a, mut y_a) = (0,0);
    let (mut x_b, mut y_b) = (0,0);
    houses.insert((x,y));
    houses2.insert((x,y));

    for direction in std::fs::read_to_string("input").unwrap().chars() {

        match direction {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!(),
        }

        turn = !turn;
        if turn {
            println!("Santa");
            match direction {
                '^' => y_a -= 1,
                'v' => y_a += 1,
                '<' => x_a -= 1,
                '>' => x_a += 1,
                _ => panic!(),
            }
        } else {
            println!("Robot");
            match direction {
                '^' => y_b -= 1,
                'v' => y_b += 1,
                '<' => x_b -= 1,
                '>' => x_b += 1,
                _ => panic!(),
            };
        }

        houses.insert((x,y));
        houses2.insert((x_a,y_a));
        houses2.insert((x_b,y_b));
    }
    println!("Part 1: {}", houses.len());
    println!("Part 2: {}", houses2.len());
}

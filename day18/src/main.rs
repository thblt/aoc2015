const DIM: usize = 100;

type Panel = [[bool; DIM]; DIM];

fn show(panel: &Panel) {
    for y in 0..DIM {
        for x in 0..DIM {
            print!("{}", if panel[x][y] { '#' } else { '.' });
        }
        println!();
    }
}

fn neighbors(panel: &Panel, x: usize, y: usize) -> usize {
     let x = x as isize;
    let y = y as isize;
    let mut ret = 0;
    for nx in [x - 1, x, x + 1] {
        for ny in [y - 1, y, y + 1] {
             if (nx != x || ny != y)
                 && nx >= 0
                 && nx < DIM as isize
                 && ny >= 0
                 && ny < DIM as isize
                 && panel[nx as usize][ny as usize]
            {
                 ret += 1;
            }
        }
    }
    ret
}

fn update(read: &Panel, write: &mut Panel) {
    for x in 0..DIM {
        for y in 0..DIM {
            let nb = neighbors(read, x, y);
            write[x][y] =
 // A light which is on stays on iff 2 or 3 neighbors are on
(read[x][y] && (nb == 2 || nb == 3))
    // A light which is off turns on if exactly 3 neighbors are on
    || (!read[x][y] && (nb == 3));
        }
    }
}

fn main() {
    let mut init = [[false; DIM]; DIM];

    let mut x = 0;
    let mut y = 0;
    for c in std::fs::read_to_string("input").unwrap().chars() {
        match c {
            '\n' => {
                y += 1;
                x = 0;
            }
            '#' => {
                init[x][y] = true;
                x += 1;
            }
            '.' => {
                init[x][y] = false;
                x += 1;
            }
            _ => panic!(),
        }
    }

    let mut panel_a = init.clone();
    let mut panel_b = [[false;100];100];

    let mut read = &mut panel_a;

    let mut write = &mut panel_b;
    for _ in 0..100 {
        update(&read, &mut write);
        (read, write) = (write, read);
    }

    let mut ret = 0;
    for x in 0..DIM {
        for y in 0..DIM {
            if read[x][y] {
                ret += 1;
            }
        }
    }
    println!("Part 1: {ret}");

    panel_a = init.clone();

    read = &mut panel_a;
    write = &mut panel_b;
    for _ in 0..100 {
        read[0][0] = true;
        read[0][99] = true;
        read[99][0] = true;
        read[99][99] = true;
        update(&read, &mut write);
        (read, write) = (write, read);
    }

    read[0][0] = true;
    read[0][99] = true;
    read[99][0] = true;
    read[99][99] = true;

    let mut ret = 0;
    for x in 0..DIM {
        for y in 0..DIM {
            if read[x][y] {
                ret += 1;
            }
        }
    }
    println!("Part 2: {ret}");
}

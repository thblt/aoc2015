const PW: &str = "hxbxwxba";

type Pw = Vec<char>;

fn validate(pw: &Pw) -> bool{
    let mut test_inc = false;
    let mut test_rep = 0;
    let mut prev_same = 0;
    let mut prev_inc = 0;
    let mut inc_count= 0;
    for c in pw.iter().map(|c| *c as u8) {
        if c == prev_same {
            test_rep += 1;
            prev_same = 0;
        } else {
            prev_same = c
        }

        if c == prev_inc + 1 {
            inc_count += 1;
            test_inc |= inc_count == 3;
        } else {
            inc_count = 1;
        }
        prev_inc = c;
    }
    test_inc && test_rep >= 2
}

fn step(c: char) -> (bool, char) {
    let ret = (c as u8 + 1) as char;
    if c == 'z' {
        (true, 'a')
    }  else if ret == 'i' || ret == 'o' || ret == 'l' {
        step(ret)
     } else {
         (false, ret)
     }
}

fn increment(pw: &mut Pw) {
    fn increment(pw: &mut Pw, index: usize) {
        let (looped, value) = step(pw[index]);
        pw[index] = value;
        if looped {
            increment(pw, index-1);
        }

    }
    increment(pw, pw.len()-1);
}

fn main() {
    let mut pw = PW.chars().collect::<Vec<char>>();
    while !(validate(&pw))
        {increment(&mut pw);}
    println!("Part 1: {}", pw.iter().collect::<String>());
    increment(&mut pw);
    while !(validate(&pw))
        {increment(&mut pw);}
    println!("Part 2: {}", pw.iter().collect::<String>());

}

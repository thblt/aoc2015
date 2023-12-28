use md5::{Md5, Digest};

pub fn md5(message: &[u8]) {
    // Prepare constants;
    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, //
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, //
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,//
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21 ];
    let mut k: [u32; 64] = [0;64];
    let two32 = (2.0 as f64).powf(32.0);
    for i in 0..64 {
        k[i] = (two32 * (i as f64 + 1.0).sin().abs()).floor() as u32;
    }
    // Pad message
    let mut message = message.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0x00);
    }
    for idx in (0..message.len()).step_by(64) {
        println!("{idx}, {}", message[idx]);
    }
}

fn main() {
    let prefix = "ckczppom".to_string();
    // create a Md5 hasher instance

    for i in 0.. {
        let mut hasher = Md5::new();
        let cand = prefix.clone() + &i.to_string();
        hasher.update(&cand);
        let result = hasher.finalize();
        if result[0] == 0 && result[1]== 0 && result[2] & 0b11110000 == 0
        {
            println!("Part 1: {cand}");
        }
        if result[0] == 0 && result[1]== 0 && result[2]  == 0
{
    println!("Part 2: {cand}");
    break
}
    }
}

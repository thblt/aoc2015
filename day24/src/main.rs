// This is a stupid solution but I'm leaving it as is.  You don't need
// to separate anything, just to (1) sum all numbers, divide by 3 for
// part 1, by 4 for part 2 (2) find the minimum number of numbers
// required to sum to this total, (3) enumerate possibilities,
// calculate their product, find the lowest.

use std::collections::HashSet;

const GOAL: u64 = (512*3)/3;

fn main() {
    let mut qe: u64 = u64::MAX;
    let numbers: Vec<u64> = vec![
        1, 2, 3, 5, 7, 13, 17, 19, 23, 29, 31, 37, 41, 43, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113,
    ];
    for a in numbers.iter().rev() {
        for b in numbers.iter().rev() {
            for c in numbers.iter().rev() {
                for d  in numbers.iter().rev() {
                    for e in numbers.iter().rev() {
                        let mut test = HashSet::new();
                        test.insert(a);
                        test.insert(b);
                        test.insert(c);
                        test.insert(d);
                        test.insert(e);
                        let sum = test.iter().copied().sum::<u64>();
                        if sum > GOAL { continue }
                        let rem = GOAL - sum;
                            if rem == 0 {
                                let this_qe: u64 = test.iter().copied().product::<u64>();
                                if this_qe < qe {
                                    println!("New: {this_qe}");
                                    qe = this_qe;
                                }
                            } else {
                                if numbers.iter().any(|n| *n == rem) && !test.contains(&rem) {
                                    let this_qe: u64 = rem as u64 * test.iter().copied().product::<u64>();
                                    if this_qe < qe {
                                        println!("New: {this_qe}");
                                        qe = this_qe;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

/// Part 2 *can* be brute-forced, but you don't need to.  The trick is
/// that you don't need to insert yourself in the problem's setup as
/// another guest.  Inserting yourself between a pair of guests just
/// substracts their contribution to the overall happiness.  So for
/// each setting, the best way to seat yourself is between the two
/// guests with the lower contribution to overall hapiness.  This is
/// what Table::score_seating does.

use std::{collections::HashMap, cmp::{max, min}};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Table {
    // nodes: HashMap<usize, Node>,
    /// Distances from city to city
    happiness: HashMap<(usize, usize), i64>,
    names: HashMap<usize, String>,
    ids: HashMap<String, usize>,
}

impl Table {
    fn new() -> Self {
        Self {
            happiness: HashMap::new(),
            names: HashMap::new(),
            ids: HashMap::new(),
        }
    }

    /// Return the id of the machine called name, creating it if
    /// needed.
    fn intern(&mut self, name: &str) -> usize {
        if let Some(id) = self.ids.get(name) {
            *id
        } else {
            let id = self.names.len();
            self.names.insert(id, name.to_string());
            self.ids.insert(name.to_string(), id);
            id
        }
    }

    /// Score this seating, returning two scores: the seatings's, and
    /// the seating if I was to sit between the two people with the
    /// worst contribution to overall happiness.
    fn score_seating(&self, seating: &Vec<usize>) -> (i64, i64) {
        let mut worst = self.happiness[&(seating[0], *seating.last().unwrap())]
            + self.happiness[&(*seating.last().unwrap(), seating[0])];
        let mut ret = worst;
        for i in 1..seating.len() {
            let this = self.happiness[&(seating[i-1], seating[i])] +
                self.happiness[&(seating[i], seating[i-1])];
            worst = min(worst, this);
            ret += this;
        }
        (ret, ret-worst)
    }

    fn best_seating(&self) -> (i64, i64) {
        let cities = &self.names.keys().copied().collect::<Vec<usize>>();
        self.do_walk(cities, vec!())
    }

    fn do_walk(&self, rem: &[usize], seating: Vec<usize>) -> (i64, i64) {
        if rem.is_empty() {
            self.score_seating(&seating)
        } else {
            let mut best1 = i64::MIN;
            let mut best2 = i64::MIN;
            for next in rem {
                let (this1, this2) = self.do_walk(
                    &rem.iter()
                        .copied()
                        .filter(|c| c != next)
                        .collect::<Vec<usize>>(),
                    seating.iter().chain(std::iter::once(next)).copied().collect::<Vec<usize>>()
                );
                best1 = max(this1,best1);
                best2 = max(this2,best2);
            }
            (best1,best2)
        }
    }
}

fn main() {
    let mut table = Table::new();
    for line in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        let (who, gain, mut count, other) = sscanf::sscanf!(line, "{String} would {String} {i64} happiness units by sitting next to {String}.").unwrap();
        let who = table.intern(&who);
        let other = table.intern(&other);
        if gain == "lose" {
            count = -count;
        }
        table.happiness.insert((who, other), count);
    }

    let (result1,result2) = table.best_seating();
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}

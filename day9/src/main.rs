use std::{collections::HashMap, cmp::{max, min}};

#[derive(Debug, Eq, PartialEq, Clone)]
struct World {
    // nodes: HashMap<usize, Node>,
    /// Distances from city to city
    dists: HashMap<(usize, usize), u64>,
    names: HashMap<usize, String>,
    ids: HashMap<String, usize>,
}

impl World {
    fn new() -> Self {
        Self {
            dists: HashMap::new(),
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

    fn walk(&self) -> (u64,u64) {
        let cities = &self.names.keys().copied().collect::<Vec<usize>>();
        self.do_walk(None, cities, 0)
    }

    fn do_walk(&self, pos: Option<usize>, rem: &[usize], score: u64) -> (u64,u64) {
        if rem.is_empty() {
            (score, score)
        } else {
            let mut best = u64::MAX;
            let mut worst = 0;
            for next in rem {
                let score =
                    if let Some(pos) = pos {
                        score + self.dists[&(pos, *next)]
                    } else {
                        score
                    };
                let (new_best, new_worst) = self.do_walk(
                    Some(*next),
                    &rem.iter()
                        .copied()
                        .filter(|c| c != next)
                        .collect::<Vec<usize>>(),
                    score,
                );
                best = min(best, new_best);
                worst = max(worst, new_worst);
            }
            (best,worst)
        }
    }
}

fn main() {
    let mut world = World::new();
    for line in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        let (from, to, dist) = sscanf::sscanf!(line, "{String} to {String} = {u64}").unwrap();
         let from = world.intern(&from);
        let to = world.intern(&to);
         world.dists.insert((from, to), dist);
        world.dists.insert((to, from), dist);
    }
    let (best,worst) = world.walk();
    println!("Part 1: {best}");
    println!("Part 2: {worst}");
}

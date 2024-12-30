// Nothing special here. The only trick is that we combine armor and rings into
// a single big list that contains all possibilities. We add a special No armor
// armor for zero gold and zero effects, and a No ring ring. We begin by extending
// the rings list with pairs of rings, then produce a list with all armor + rings
// combinations.  So the equip() function is very simple:  It begins by choosing
// each weapon.  If the weapon is less expensive than the minimum amount of gold
// that must be spent (initially zero, then the amount that made us lose the
// previous round + 1), it adds the cheapest accessory that, combined with the
// weapon, reaches the minimum gold (an accessory is any combination of zero or
// one armor, and zero, one or two rings.  The only subtlety is that multiple
// combinations have the same price, so they must all be tested.

use std::ops::Add;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Object {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Object {
    fn new(cost: i32, damage: i32, armor: i32) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

impl Add<Object> for Object {
    type Output = Object;

    fn add(self, rhs: Object) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
}

fn fight_step(attacker: &Player, defender: &mut Player) -> bool {
    let damage = std::cmp::max(1, attacker.damage - defender.armor);
    defender.hp = std::cmp::max(0, defender.hp - damage);
    defender.hp == 0
}

fn fight(mut player: Player, mut boss: Player) -> bool {
    loop {
        if fight_step(&player, &mut boss) {
            return true;
        }
        if fight_step(&boss, &mut player) {
            return false;
        }
    }
}

fn build_shop() -> (Vec<Object>, Vec<Object>) {
    let mut weapons = vec![
        Object::new(8, 4, 0),
        Object::new(10, 5, 0),
        Object::new(25, 6, 0),
        Object::new(40, 7, 0),
        Object::new(74, 8, 0),
    ];
    let armor = vec![
        Object::new(0, 0, 0),
        Object::new(13, 0, 1),
        Object::new(31, 0, 2),
        Object::new(53, 0, 3),
        Object::new(75, 0, 4),
        Object::new(102, 0, 5),
    ];
    let mut rings = vec![
        Object::new(25, 1, 0),
        Object::new(50, 2, 0),
        Object::new(100, 3, 0),
        Object::new(20, 0, 1),
        Object::new(40, 0, 2),
        Object::new(80, 0, 3),
    ];
    // Combine rings
    let len = rings.len();
    for r0 in 0..len {
        for r1 in r0 + 1..len {
            rings.push(rings[r0] + rings[r1]);
        }
    }
    rings.push(Object::new(0, 0, 0));
    let mut accessories = vec![];
    for a in armor {
        for r in &rings {
            accessories.push(a + *r);
        }
    }
    weapons.sort_by_key(|o| o.cost);
    accessories.sort_by_key(|o| o.cost);
    accessories.dedup();

    (weapons, accessories)
}

// Find the cheapest equipment you can find for at least min_gold.  Because
// there may be multiple combinations for the same cost, this function returns a
// Vec of combined objects.
fn equip(weapons: &[Object], accessories: &[Object], min_gold: i32) -> Vec<Object> {
    let mut candidates = vec![];
    for weapon in weapons {
        if weapon.cost >= min_gold {
            candidates.push(*weapon);
        } else {
            let rem_gold = min_gold - weapon.cost;
            if let Some(cheapest) = accessories.iter().find(|o| o.cost >= rem_gold) {
                candidates.extend(
                    accessories
                        .iter()
                        .filter(|o| o.cost == cheapest.cost)
                        .map(|o| *weapon + *o),
                );
            }
        }
    }
    if !candidates.is_empty() {
        let cheapest = candidates.iter().map(|o| o.cost).min().unwrap();
        candidates.retain(|o| o.cost == cheapest);
    }
    candidates
}
fn main() {
    let (weapons, accessories) = build_shop();
    let boss = Player {
        hp: 109,
        damage: 8,
        armor: 2,
    };
    let mut part1 = None;
    let mut part2 = 0;
    let mut gold = 0;
    loop {
        let equipment = equip(&weapons, &accessories, gold + 1);
        if equipment.is_empty() {
            break;
        }
        gold = equipment[0].cost;

        for attempt in equipment {
            let player = Player {
                hp: 100,
                damage: attempt.damage,
                armor: attempt.armor,
            };

            if fight(player, boss) {
                if part1.is_none() {
                    part1 = Some(gold);
                }
            } else {
                part2 = gold;
            }
        }
    }
    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {part2}");
}

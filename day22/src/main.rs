/* I actually solved this one by hand.  The program starts in interactive mode,
you build your strategy by choosing which spell to cast at which moment, and the
program immediately display the full battle.  Type `hard` to enable hard mode
(part 2) */

use std::cmp::max;

const MISSILE: Spell = Spell {
    cost: 53,
    timer: 0,
    damage: 4,
    heal: 0,
    armor: 0,
    mana: 0,
};
const DRAIN: Spell = Spell {
    cost: 73,
    timer: 0,
    damage: 2,
    heal: 2,
    armor: 0,
    mana: 0,
};
const SHIELD: Spell = Spell {
    cost: 113,
    timer: 6,
    damage: 0,
    heal: 0,
    armor: 7,
    mana: 0,
};
const POISON: Spell = Spell {
    cost: 173,
    timer: 6,
    damage: 3,
    heal: 0,
    armor: 0,
    mana: 0,
};
const RECHARGE: Spell = Spell {
    cost: 229,
    timer: 5,
    damage: 0,
    heal: 0,
    armor: 0,
    mana: 101,
};
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Spell {
    cost: i32,
    // Duration. If null, effect is immediate, otherwise install a timed effect.
    timer: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana: i32,
}

enum Outcome {
    Victory(usize),
    Defeat(usize),
    Illegal(usize),
}

impl Outcome {
    fn from_hps(nth: usize, human: i32, boss: i32) -> Self {
        assert!(!(human <= 0 && boss <= 0));
        if boss <= 0 {
            Self::Victory(nth)
        } else if human <= 0 {
            Self::Defeat(nth)
        } else {
            unreachable!()
        }
    }
}
impl Spell {
    fn name(&self) -> String {
        match self.cost {
            53 => "Magic Missile",
            73 => "Drain",
            113 => "Shield",
            173 => "Poison",
            229 => "Recharge",
            _ => panic!(),
        }
        .to_string()
    }
    fn abbr(&self) -> String {
        match self.cost {
            53 => "M",
            73 => "D",
            113 => "S",
            173 => "P",
            229 => "R",
            _ => panic!(),
        }
        .to_string()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct State {
    boss_hp: i32,
    boss_damage: i32,

    hp: i32,
    mana: i32,
    armor: i32,

    mana_spent: i32,

    effects: Vec<Spell>,
}

impl State {
    fn can_cast(&self, spell: &Spell) -> bool {
        !self.is_running(spell) && self.mana >= spell.cost
    }

    fn is_running(&self, spell: &Spell) -> bool {
        self.effects
            .iter()
            .any(|eff| eff.timer > 0 && eff.cost == spell.cost)
    }

    fn apply_spell(&mut self, spell: &Spell) {
        self.hp += spell.heal;
        self.mana += spell.mana;
        self.armor += spell.armor;
        self.boss_hp -= spell.damage;
    }
}

fn print_game(game: &[State], actions: &[Spell], outcome: Outcome) {
    fn format_effects(state: &State) -> String {
        let mut ret = String::from("(");
        let mut first = true;
        for eff in &state.effects {
            if first {
                first = false;
            } else {
                ret.push('·')
            }
            // ret.push_str(eff.abbr());
            ret.push_str(&format!("{}{}", eff.abbr(), eff.timer));
        }
        ret.push(')');
        ret
    }
    let mut actions = actions.iter();
    let mut human;
    for (nth, state) in game.iter().enumerate() {
        if nth % 2 == 0 {
            human = String::new();
            human.push_str(&format!(
                "{nth:2} HUMAN: {:15} hp={:<3} mana={:<4} ",
                actions.next().unwrap().name(),
                state.hp,
                state.mana,
            ));
            human.push_str(&format_effects(state));
            human.push(' ');
            print!("{human:.<75}");
        } else {
            print!(
                " {nth:2} BOSS: hp={}, human's armor: {} ",
                state.boss_hp, state.armor,
            );
            println!("{}", format_effects(state));
        }
    }
    if game.len() % 2 == 1 {
        println!();
    }
    match outcome {
        Outcome::Victory(nth) => print!("=> Victory at turn {nth}! "),
        Outcome::Defeat(nth) => print!("=> Defeat at turn {nth}! "),
        Outcome::Illegal(nth) => print!("=> Illegal move at turn {nth}! "),
    }
    println!("You used up {} mana.", game.last().unwrap().mana_spent);
}

fn run_game(mut state: State, actions: &[Spell], hard_mode: bool) -> (Outcome, Vec<State>) {
    let mut game = vec![];
    let mut actions = actions.iter();
    for nth in 0.. {
        // Reset magical armor.
        state.armor = 0;
        // Handle hard mode
        if hard_mode && nth % 2 == 0 {
            state.hp -= 1;
            if state.hp <= 0 {
                return (Outcome::Defeat(nth), game);
            }
        }
        // Clear expired effects (we do that the turn after so print_game can show effects at timer 0)
        state.effects.retain(|eff| eff.timer != 0);
        // Apply spell effects
        for i in 0..state.effects.len() {
            let spell = state.effects[i];
            state.apply_spell(&spell);
            state.effects[i].timer -= 1
        }
        if state.boss_hp <= 0 || state.hp <= 0 {
            game.push(state.clone());
            return (Outcome::from_hps(nth, state.hp, state.boss_hp), game);
        }
        if nth % 2 == 0 {
            // ############
            // ## PLAYER ##
            // ############
            let spell = actions.next().unwrap();
            // Cast the spell the player chose, if we can.
            if state.can_cast(spell) {
                state.mana -= spell.cost;
                state.mana_spent += spell.cost;
                if spell.timer > 0 {
                    state.effects.push(*spell);
                } else {
                    state.apply_spell(spell);
                }
            } else {
                // Return a failure, because we can't cast the chosen spell;
                return (Outcome::Illegal(nth), game);
            }
        } else {
            // ##########
            // ## BOSS ##
            // ##########
            let damage = max(1, state.boss_damage - state.armor);
            state.hp -= damage
        }
        game.push(state.clone());
        if state.boss_hp <= 0 || state.hp <= 0 {
            return (Outcome::from_hps(nth, state.hp, state.boss_hp), game);
        }
    }
    unreachable!()
}

fn interactive(init_state: State) {
    fn run_and_print(init_state: State, moves: &[Spell], hard_mode: bool) {
        let (outcome, game) = run_game(init_state.clone(), moves, hard_mode);
        print_game(&game, moves, outcome);
    }

    let mut moves = vec![MISSILE; 100];
    let mut hard_mode = false;
    run_and_print(init_state.clone(), &moves, hard_mode);
    println!("Type 'hard' to toggle hard mode, or a command like 22M to choose to cast magic missile at turn 22.");
    println!("Spells are Missile (M), Drain (D), Shield (S), Poison (P) and Recharge (R)");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line == "hard" {
            hard_mode = !hard_mode;
            println!("hard mode: {hard_mode}");
            run_and_print(init_state.clone(), &moves, hard_mode);
        } else if let Some((pos, spell)) = read_command(line) {
            moves[pos / 2] = spell;
            println!(
                "At move {pos}, replaced {} by {}.",
                moves[pos / 2].name(),
                spell.name()
            );
            run_and_print(init_state.clone(), &moves, hard_mode);
        }
    }
}
fn main() {
    let init_state = State {
        boss_hp: 71,
        boss_damage: 10,

        hp: 50,
        mana: 500,
        armor: 0,
        mana_spent: 0,

        effects: vec![],
    };
    interactive(init_state);
}

fn read_command(cmd: &str) -> Option<(usize, Spell)> {
    let mut cmd = cmd.chars();
    let mut turn = 0;
    let mut c = '0';
    while c.is_ascii_digit() {
        turn *= 10;
        turn += c.to_digit(10).unwrap() as usize;
        if let Some(next) = cmd.next() {
            c = next;
        } else {
            println!("No spell?");
            return None;
        }
    }
    if turn % 2 != 0 {
        println!("Play at YOUR turn!");
        return None;
    }

    let spell = match c {
        'M' => Some(MISSILE),
        'D' => Some(DRAIN),
        'S' => Some(SHIELD),
        'P' => Some(POISON),
        'R' => Some(RECHARGE),
        _ => {
            println!("No such spell");
            None
        }
    }?;
    if cmd.next().is_none() {
        Some((turn, spell))
    } else {
        println!("Extra input!");
        None
    }
}

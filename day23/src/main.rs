struct Machine {
    program: Program,
    index: usize,
    a: u64,
    b: u64,
}

impl Machine {
    fn new(program: Program) -> Self {
        Machine {
            program,
            index: 0,
            a: 0,
            b: 0,
        }
    }

    fn update_register(&mut self, reg: Reg, f: fn(u64) -> u64) {
        match reg {
            Reg::A => self.a = f(self.a),
            Reg::B => self.b = f(self.b),
        }
    }

    fn test_register(&mut self, reg: Reg, f: fn(u64) -> bool) -> bool {
        match reg {
            Reg::A => f(self.a),
            Reg::B => f(self.b),
        }
    }

    fn run(&mut self) {
        loop {
            let mut next_index = self.index as isize + 1;
            match self.program[self.index] {
                Op::Hlf(r) => self.update_register(r, |x| x / 2),
                Op::Tpl(r) => self.update_register(r, |x| x * 3),
                Op::Inc(r) => self.update_register(r, |x| x + 1),
                Op::Jmp(v) => next_index = self.index as isize + v,
                Op::Jie(r, v) => {
                    if self.test_register(r, |r| r % 2 == 0) {
                        next_index = self.index as isize + v
                    }
                }
                Op::Jio(r, v) => {
                    if self.test_register(r, |r| r == 1) {
                        next_index = self.index as isize + v
                    }
                }
            }
            if next_index > 0 && next_index < self.program.len() as isize {
                self.index = next_index as usize;
            } else {
                break;
            }
        }
    }
}

type Program = Vec<Op>;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reg {
    A,
    B,
}

impl Reg {
    fn read(name: &str) -> Reg {
        match name {
            "a" => Reg::A,
            "b" => Reg::B,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(isize),
    Jie(Reg, isize),
    Jio(Reg, isize),
}

impl Op {
    fn read(asm: &str) -> Self {
        let (op, args) = asm.split_once(' ').unwrap();
        use Op::*;
        let reg_offset = args.split_once(", ");
        match op {
            "hlf" => Hlf(Reg::read(args)),
            "tpl" => Tpl(Reg::read(args)),
            "inc" => Inc(Reg::read(args)),
            "jmp" => Jmp(args.parse::<isize>().unwrap()),
            "jie" => Jie(
                Reg::read(reg_offset.unwrap().0),
                reg_offset.unwrap().1.parse::<isize>().unwrap(),
            ),
            "jio" => Jio(
                Reg::read(reg_offset.unwrap().0),
                reg_offset.unwrap().1.parse::<isize>().unwrap(),
            ),
            _ => panic!("Bad instruction"),
        }
    }
}

fn main() {
    let mut program = Program::new();

    for loc in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        program.push(Op::read(loc));
    }

    let mut machine = Machine::new(program);
    machine.run();
    println!("Part 1: {}", machine.b);

    machine.a = 1;
    machine.b = 0;
    machine.index = 0;
    machine.run();
    println!("Part 2: {}", machine.b);
}

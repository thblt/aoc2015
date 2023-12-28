use std::{collections::HashMap, usize};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pin {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Circuit {
    nodes: HashMap<usize, Node>,
    names: HashMap<usize, String>,
    ids: HashMap<String, usize>,
    ready: bool,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            nodes: HashMap::new(),
            names: HashMap::new(),
            ids: HashMap::new(),
            ready: false,
        }
    }

    /// Return the id of the machine called name, creating it if
    /// needed.
    fn intern(&mut self, name: &str) -> usize {
        if let Some(id) = self.ids.get(name) {
            *id
        } else {
            let id = self.nodes.len();
            let node = Node::new(Op::Pass);
            self.nodes.insert(id, node);
            self.names.insert(id, name.to_string());
            self.ids.insert(name.to_string(), id);
            id
        }
    }

    fn set_op(&mut self, node: usize, op: Op) {
        self.nodes.get_mut(&node).unwrap().op = op;
    }

    /// Read an argument to gate target from a string, storing it in
    /// the gate if a literal value, mapping target to the output of
    /// the other gate otherwise.
    fn read_arg(&mut self, target: usize, pin: Pin, arg: &str) {
        if let Ok(value) = arg.parse::<u16>() {
            self.recv(target, pin, value);
        } else {
            let source = self.intern(arg);
            self.nodes.get_mut(&source).unwrap().outputs.push((target, pin));
        }
    }

    // Receive a value on the left pin of node.
    fn recv(&mut self, node: usize, pin: Pin, value: u16) {
        match pin {
            Pin::Left => self.nodes.get_mut(&node).unwrap().lhs = Some(value),
            Pin::Right => self.nodes.get_mut(&node).unwrap().rhs = Some(value),
        }
        if self.ready {
            self.try_send(node);
        }
    }

    /// If that node has all its inputs populated, compute its result,
    /// store it in lhs and propagate it back to its outputs.
    fn try_send(&mut self, id: usize) -> bool {
        let node = &mut self.nodes.get_mut(&id).unwrap();
        if !node.done && node.lhs.is_some() && (node.is_unary() || node.rhs.is_some()) {
            let lhs = node.lhs.unwrap();
            let rhs = || node.rhs.unwrap();
            let result = match node.op {
                Op::Pass => lhs,
                Op::And => lhs & rhs(),
                Op::Or => lhs | rhs(),
                Op::Not => !lhs,
                Op::LShift => lhs << rhs(),
                Op::RShift => lhs >> rhs(),
            };
            // Put result in lhs
            node.lhs = Some(result);
            node.done = true;
            for (target, pin) in node.outputs.clone() {
                self.recv(target, pin, result);
            }
            return true
        }
        false
    }

    fn run(&mut self) {
        self.ready = true;
        let ids = self.nodes.keys().copied().collect::<Vec<usize>>();
        for id in ids {
            self.try_send(id);
        }
    }

    fn read_node(&self, name: &str) -> Option<u16>{
        let id = self.ids[name];
        if let Some(Node { done: true, lhs: Some(value), ..}) = self.nodes.get(&id) {
            Some(*value)
        } else
        {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Op {
    Pass,
    And,
    Or,
    Not,
    LShift,
    RShift,
}

type Output = (usize, Pin);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    lhs: Option<u16>,
    rhs: Option<u16>,
    outputs: Vec<Output>,
    op: Op,
    done: bool,
}

impl Node {
    fn new(op: Op) -> Node {
        Node {
            lhs: None,
            rhs: None,
            outputs: vec![],
            op,
            done: false,
        }
    }

    fn is_unary(&self) -> bool {
        match self.op {
            Op::Pass => true,
            Op::Not => true,
            _ => false,
        }
    }
}

fn main() {
    let mut circuit = Circuit::new();
    for line in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        let line = line
            .split(' ')
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        let (op, lhs, rhs, name) = match line[1].as_str() {
            "RSHIFT" => (Op::RShift, &line[0], Some(&line[2]), &line[4]),
            "LSHIFT" => (Op::LShift, &line[0], Some(&line[2]), &line[4]),
            "OR" => (Op::Or, &line[0], Some(&line[2]), &line[4]),
            "AND" => (Op::And, &line[0], Some(&line[2]), &line[4]),
            _ if &line[0] == "NOT" => (Op::Not, &line[1], None, &line[3]),
            _ if &line[1] == "->" => (Op::Pass, &line[0], None, &line[2]),
            _ => panic!("syntax error"),
        };
        let id = circuit.intern(name);
        circuit.set_op(id, op);
        circuit.read_arg(id, Pin::Left, lhs);
        if let Some(rhs) = rhs {
            circuit.read_arg(id, Pin::Right, rhs);
        }
    }

    let mut circuit2 = circuit.clone();

    circuit.run();
    let result1 = circuit.read_node("a").unwrap();
    println!("Part 1: {}", result1);

    let b_id = circuit2.intern("b");
    circuit2.set_op(b_id, Op::Pass);
    circuit2.recv(b_id, Pin::Left, result1);
    circuit2.run();

    println!("Part 2: {}", circuit2.read_node("a").unwrap());

}

use std::collections::VecDeque;
use std::fmt;
use std::iter;
use std::rc::Rc;

use rand::{Rng, Rand};

#[derive(Clone, Copy, Debug)]
pub enum Symbol {Zero, One}

pub const NUM_SYMBOLS: usize = 2;
const SYMBOLS: [Symbol; NUM_SYMBOLS] =
    [Symbol::Zero, Symbol::One];

#[derive(Debug)]
pub struct Tape {
    data: VecDeque<u32>,
    start_point: isize,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {Left, Right}

#[derive(Clone, Copy, Debug)]
pub struct StateID(pub u32);

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Halt,
    Transition {
        write: Symbol,
        movement: Direction,
        next_state: StateID,
    },
}

type TransitionRule = [Action; NUM_SYMBOLS];

#[derive(Debug)]
pub struct TuringMachine {
    initial_state: StateID,
    transition_rules: Vec<TransitionRule>,
}

#[derive(Debug)]
pub struct TuringMachineComputation {
    is_halted: bool,
    tape: Tape,
    tape_head: i32,
    cur_state: StateID,
    turing_machine: Rc<TuringMachine>,
}

impl Direction {
    fn to_int(self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl Symbol {
    fn from_int(n: u32) -> Symbol {
        SYMBOLS[n as usize]
    }

    fn to_int(self) -> u32 {
        match self {
            Symbol::Zero => 0,
            Symbol::One => 1,
        }
    }
}

impl Tape {
    pub fn read_at(&self, index: i32) -> Symbol {
        let offset = index & 31;
        let array_index = (index >> 5) as isize + self.start_point;
        assert!(0 <= offset && offset < 32, "offset {}", offset);
        if array_index < 0 {
            Symbol::Zero
        } else {
            self.data.get(array_index as usize)
                     .map_or(Symbol::Zero,
                             |n| Symbol::from_int((n >> offset) & 1))
        }
    }

    pub fn write_at(&mut self, index: i32, new_val: Symbol) {
        let offset = index & 31;
        let mut array_index = (index >> 5) as isize + self.start_point;

        if array_index < 0 {
            // std Deque doesn't have convenient way of extending at the front
            for _ in 0..-array_index {
                self.data.push_front(0);
                self.start_point += 1;
                array_index += 1;
            }
        }

        if array_index >= self.data.len() as isize {
            let extend_len = array_index as usize + 1 - self.data.len();
            self.data.extend(iter::repeat(0).take(extend_len));
        }

        let mask = 1 << offset;
        let prev_bits = self.data[array_index as usize];
        let new_bits = (prev_bits & !mask) | (new_val.to_int() << offset);
        self.data[array_index as usize] = new_bits;
    }
}

impl Default for Tape {
    fn default() -> Self {
        Tape {
            data: VecDeque::new(),
            start_point: 0,
        }
    }
}

impl TuringMachine {
    pub fn initial_state(&self) -> StateID {
        self.initial_state
    }

    pub fn num_states(&self) -> usize {
        self.transition_rules.len()
    }

    pub fn lookup_action(&self, state: StateID, symb: Symbol) -> Action {
        self.transition_rules[state.0 as usize][symb as usize]
    }
}

impl TuringMachineComputation {
    pub fn start(turing_machine: Rc<TuringMachine>) -> Self {
        TuringMachineComputation {
            is_halted: false,
            tape: Tape::default(),
            tape_head: 0,
            cur_state: turing_machine.initial_state,
            turing_machine: turing_machine,
        }
    }

    pub fn turing_machine(&self) -> &TuringMachine {
        &self.turing_machine
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn tape(&self) -> &Tape {
        &self.tape
    }

    pub fn tape_head_position(&self) -> i32 {
        self.tape_head
    }

    pub fn current_state(&self) -> StateID {
        self.cur_state
    }

    fn read_head(&self) -> Symbol {
        self.tape.read_at(self.tape_head)
    }

    fn write_head(&mut self, new_symb: Symbol) {
        self.tape.write_at(self.tape_head, new_symb);
    }

    fn move_dir(&mut self, dir: Direction) {
        self.tape_head += dir.to_int();
    }

    fn next_action(&self) -> Action {
        self.turing_machine.lookup_action(self.cur_state, self.read_head())
    }

    fn perform_action(&mut self, action: Action) {
        match action {
            Action::Halt => {self.is_halted = true;},
            Action::Transition {write, movement, next_state} => {
                self.write_head(write);
                self.move_dir(movement);
                self.cur_state = next_state;
            }
        }
    }

    pub fn step(&mut self) -> bool {
        if !self.is_halted() {
            // Lexical borrows *grumbles*
            let next_action = self.next_action();
            self.perform_action(next_action);
        }
        return self.is_halted();
    }
}

impl Rand for Symbol {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let symbols = [Symbol::Zero, Symbol::One];
        symbols[rng.gen_range(0, 2)]
    }
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let directions = [Direction::Left, Direction::Right];
        directions[rng.gen_range(0, 2)]
    }
}

fn random_action<R: Rng>(rng: &mut R, num_states: u32) -> Action {
    let next_state_id = rng.gen_range(0, num_states+1);
    if next_state_id == num_states {
        return Action::Halt;
    }
    let next_state = StateID(next_state_id);
    let dir = rng.gen();
    let new_symbol = rng.gen();
    Action::Transition {
        write: new_symbol,
        movement: dir,
        next_state: next_state,
    }
}

pub fn random_turing_machine<R: Rng>(rng: &mut R, num_states: u32) ->
    TuringMachine {

    let mut transition_rules = Vec::with_capacity(num_states as usize);
    for _ in 0..num_states {
        let rule =
            [random_action(rng, num_states),
             random_action(rng, num_states)];
        transition_rules.push(rule);
    }

    TuringMachine {
        initial_state: StateID(0),
        transition_rules: transition_rules,
    }
}

impl fmt::Display for StateID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S{}", self.0)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir_char: char = match *self {
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{}", dir_char)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Halt => write!(f, "H"),
            Action::Transition {
                write: symb,
                movement: dir,
                next_state: state,
            } => write!(f, "{}{}{}", symb.to_int(), dir, state),
        }
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of states: {}\n", self.num_states())?;
        write!(f, "Initial state: {}\n", self.initial_state())?;

        for i in 0..self.num_states() {
            let state = StateID(i as u32);
            let action0 = self.lookup_action(state, Symbol::Zero);
            let action1 = self.lookup_action(state, Symbol::One);

            write!(f, "    {}: 0 -> {}; 1 -> {}\n", state, action0, action1)?;
        }

        // Any better way of ending it?
        write!(f, "")
    }
}

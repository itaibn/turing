use rand::{Rng, Rand};

pub const NUM_SYMBOLS: usize = 2;

#[derive(Clone, Copy, Debug)]
pub enum Symbol {Zero, One}

// Private & will be improved
const TAPE_MIN: i32 = -10;
const TAPE_MAX: i32 = 10;
const TAPE_LEN: usize = (TAPE_MAX - TAPE_MIN + 1) as usize;

#[derive(Debug)]
pub struct Tape([Symbol; TAPE_LEN]);

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
pub struct TuringMachineComputation<'a> {
    is_halted: bool,
    tape: Tape,
    tape_head: i32,
    cur_state: StateID,
    turing_machine: &'a TuringMachine,
}

impl Direction {
    fn to_int(self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl Tape {
    fn check_index(index: i32) {
        assert!(index >= TAPE_MIN && index <= TAPE_MAX,
            "Tape head out of bounds: {}", index);
    }

    pub fn read_at(&self, index: i32) -> Symbol {
        Tape::check_index(index);
        self.0[(index - TAPE_MIN) as usize]
    }

    pub fn write_at(&mut self, index: i32, new_val: Symbol) {
        Tape::check_index(index);
        self.0[(index - TAPE_MIN) as usize] = new_val;
    }
}

impl Default for Tape {
    fn default() -> Self {
        Tape([Symbol::Zero; TAPE_LEN])
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

impl<'a> TuringMachineComputation<'a> {
    pub fn start(turing_machine: &'a TuringMachine) -> Self {
        TuringMachineComputation {
            is_halted: false,
            tape: Tape::default(),
            tape_head: 0,
            cur_state: turing_machine.initial_state,
            turing_machine: turing_machine,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
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

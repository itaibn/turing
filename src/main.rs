extern crate rand;

mod turing;

fn main() {
    let mut rng = rand::StdRng::new().unwrap();
    let turing_machine = turing::random_turing_machine(&mut rng, 10);
    let mut computation =
        turing::TuringMachineComputation::start(&turing_machine);
    computation.step();
}

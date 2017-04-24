extern crate cairo;
extern crate gtk;
extern crate rand;

mod turing;

use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use turing::{Tape, TuringMachineComputation, Symbol};

fn main() {
    let mut rng = rand::StdRng::new().unwrap();
    let turing_machine = Rc::new(turing::random_turing_machine(&mut rng, 10));
    let computation = Rc::new(RefCell::new(
        TuringMachineComputation::start(turing_machine)));
    computation.borrow_mut().step();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Turing Machine");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(650, 100);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //let tape_len = 21;
    let tm_view = gtk::DrawingArea::new();
    tm_view.set_size_request(600, 80);

    let computation_clone = computation.clone();
    tm_view.connect_draw(move |_, ctx| draw_tape(ctx, &*computation.borrow()));

    window.add(&tm_view);

    window.show_all();
    gtk::main();
}

fn draw_tape(cr: &cairo::Context, computation: &TuringMachineComputation) ->
    Inhibit {

    const CELL_HEIGHT: f64 = 30.0;
    const CELL_WIDTH: f64 = 30.0;

    let tape = computation.tape();
    for n in -10..10 {
        match tape.read_at(n) {
            Symbol::Zero => {
                cr.set_source_rgb(1.0, 1.0, 1.0);
            },
            Symbol::One => {
                cr.set_source_rgb(0.0, 0.0, 0.0);
            }
        }
        cr.rectangle(((n+10) as f64)*CELL_WIDTH, 0.0, CELL_WIDTH, CELL_HEIGHT);
        cr.fill();
    }

    Inhibit(false)
}

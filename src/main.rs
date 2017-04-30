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
    let turing_machine = Rc::new(turing::random_turing_machine(&mut rng, 30));
    let computation = Rc::new(RefCell::new(
        TuringMachineComputation::start(turing_machine)));

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("template.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("top-window").unwrap();
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let tm_view: gtk::DrawingArea = builder.get_object("tm-view").unwrap();
    let computation_clone = computation.clone();
    tm_view.connect_draw(move |_, ctx| draw_tape(ctx,
        &*computation_clone.borrow()));

    let computation_clone = computation.clone();
    let tm_view_clone = tm_view.clone();
    gtk::timeout_add(1000, move || {
        let halted = computation_clone.borrow_mut().step();
        tm_view_clone.queue_draw();
        if halted {println!("Halted")}
        Continue(!halted)
    });

    gtk::main();
}

fn draw_tape(cr: &cairo::Context, computation: &TuringMachineComputation) ->
    Inhibit {

    const CELL_HEIGHT: f64 = 30.0;
    const CELL_WIDTH: f64 = 30.0;

    let tape = computation.tape();
    for n in -10..11 {
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

    cr.set_source_rgb(1.0, 0.0, 0.0);
    cr.move_to((computation.tape_head_position() + 10) as f64 * CELL_WIDTH,
        CELL_HEIGHT/2.0);
    //cr.move_to(15.0, 10.0);
    cr.show_text(&computation.current_state().to_string());

    Inhibit(false)
}

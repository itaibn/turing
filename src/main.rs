extern crate cairo;
extern crate clap;
extern crate gtk;
extern crate rand;

mod turing;

use clap::{App, Arg};
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use turing::{TuringMachineComputation, Symbol};

// Copied from http://gtk-rs.org/tuto/closures
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

const VISIBLE_CELLS: i32 = 21;

struct GuiState {
    run: TuringMachineComputation,
    view_start: i32,
}

impl GuiState {
    fn step(&mut self) -> bool{
        if self.run.step() {return true;}
        if self.run.tape_head_position() < self.view_start {
            self.view_start -= 5;
        }
        if self.run.tape_head_position() >= self.view_start + VISIBLE_CELLS {
            self.view_start += 5;
        }
        false
    }
}

fn main() {
    let matches = App::new("Turing Machine")
                          .version("0.1")
                          .about("Turing machine simulator")
                          .arg(Arg::with_name("NUM_STATES")
                               .help("Number of states for the Turing machine")
                               .long("states")
                               .takes_value(true))
                          .arg(Arg::with_name("delay")
                               .help(
                                 "Delay between Turing machine steps")
                               .long("delay")
                               .value_name("MILLISECONDS")
                               .takes_value(true))
                          .get_matches();

    let mut rng = rand::StdRng::new().unwrap();
    let num_states = matches.value_of("NUM_STATES")
                            .map(|s| s.parse()
                                      .expect("NUM_STATES must be an integer"))
                            .unwrap_or(30);
    let delay = matches.value_of("delay")
                       .map(|s| s.parse()
                                 .expect("MILLISECONDS must be an integer"))
                       .unwrap_or(1000);

    let turing_machine = Rc::new(turing::random_turing_machine(&mut rng,
        num_states));
    let gui_state_owned = GuiState {
        run: TuringMachineComputation::start(turing_machine),
        view_start: -10,
    };
    let gui_state = Rc::new(RefCell::new(gui_state_owned));

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("template.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("top-window").unwrap();
    let tm_view: gtk::DrawingArea = builder.get_object("tm-view").unwrap();

    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    tm_view.connect_draw(clone!(gui_state => move |_, ctx| {
        draw_tape(ctx, &*gui_state.borrow());
        Inhibit(false)
    }));

    gtk::timeout_add(delay, clone!(gui_state, tm_view => move || {
        let halted = gui_state.borrow_mut().step();
        tm_view.queue_draw();
        if halted {println!("Halted")}
        Continue(!halted)
    }));

    gtk::main();
}

fn draw_tape(cr: &cairo::Context, gui_state: &GuiState) {

    const CELL_HEIGHT: f64 = 30.0;
    const CELL_WIDTH: f64 = 30.0;

    let ref computation = gui_state.run;
    let tape = computation.tape();
    for j in 0..VISIBLE_CELLS {
        let n = gui_state.view_start + j;
        match tape.read_at(n) {
            Symbol::Zero => {
                cr.set_source_rgb(1.0, 1.0, 1.0);
            },
            Symbol::One => {
                cr.set_source_rgb(0.0, 0.0, 0.0);
            }
        }
        cr.rectangle((j as f64)*CELL_WIDTH, 0.0, CELL_WIDTH, CELL_HEIGHT);
        cr.fill();
    }

    cr.set_source_rgb(1.0, 0.0, 0.0);
    cr.move_to((computation.tape_head_position() - gui_state.view_start) as f64
        * CELL_WIDTH, CELL_HEIGHT/2.0);
    //cr.move_to(15.0, 10.0);
    cr.show_text(&computation.current_state().to_string());
}

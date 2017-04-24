extern crate gtk;
extern crate rand;

mod turing;

use gtk::prelude::*;

fn main() {
    let mut rng = rand::StdRng::new().unwrap();
    let turing_machine = turing::random_turing_machine(&mut rng, 10);
    let mut computation =
        turing::TuringMachineComputation::start(&turing_machine);
    computation.step();

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Turing Machine");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(450, 100);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let tape_len = 21;
    let tm_view = gtk::DrawingArea::new();
    tm_view.set_size_request(200, 80);

    tm_view.connect_draw(|_, ctx| {
        ctx.rectangle(0., 0., 10., 10.);
        ctx.fill();
        Inhibit(false)
    });

    window.add(&tm_view);

    window.show_all();
    gtk::main();
}

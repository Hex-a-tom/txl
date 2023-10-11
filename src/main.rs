use controller::Controller;


mod ui;
mod model;
mod controller;

fn main() {
    let mut controller = Controller::new();
    controller.run().expect("Error");
}

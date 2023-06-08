mod consts;
mod network;
mod simulation;
mod ui;
mod utils;

fn main() {
    simulation::temperature::execute();
    // ui::welcome_message();
    // let option = ui::choose_option();
    // ui::proceed_to_simultaion();
    //
    // if option == "1" {
    //     simulation::temperature::execute();
    // }
}

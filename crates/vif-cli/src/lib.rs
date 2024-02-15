mod application;
mod error;

use vif_loader::setup_logging;

pub fn run_cli() {
    setup_logging();

    let mut vif = application::Vif::init();

    let res = vif.run();

    match res {
        Ok(_) => (),
        Err(e) => println!("Error: {e}"),
    }
}

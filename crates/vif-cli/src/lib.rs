mod application;

use vif_loader::setup_logging;

pub fn run_cli() {
    setup_logging();

    let mut vif = application::Vif::init();
    match vif.run() {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}

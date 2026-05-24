// App startup glue lives outside main so it can be tested.
pub mod app;
// Expose config loading as part of the engine surface.
pub mod config;
// Expose runtime process handling as part of the engine surface.
pub mod process;

use std::path::Path;

use crate::app::load_process_from_file;

// Later this will start the gRPC server and engine runtime.
fn main() {
    let path = Path::new("../examples/agents/researcher.yaml");

    match load_process_from_file(path) {
        Ok(mut process) => {
            if let Err(error) = process.load() {
                eprintln!("failed to load agent process: {}", error);
                std::process::exit(1);
            }

            println!(
                "loaded agent process: id={}, state={:?}",
                process.id(),
                process.state()
            );
        }
        Err(error) => {
            eprintln!("failed to load agent process: {:?}", error);
            std::process::exit(1);
        }
    }
}

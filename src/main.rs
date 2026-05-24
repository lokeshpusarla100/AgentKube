// App startup glue lives outside main so it can be tested.
pub mod app;
// Expose config loading as part of the engine surface.
pub mod config;
// Expose runtime process handling as part of the engine surface.
pub mod process;
// Expose the execution loop runtime.
pub mod runtime;

use std::path::Path;

use crate::app::load_process_from_file;
use crate::runtime::run_fixed_steps;

// Later this will start the gRPC server and engine runtime.
fn main() {
    let path = Path::new("../examples/agents/researcher.yaml");

    match load_process_from_file(path) {
        Ok(mut process) => {
            if let Err(error) = process.load() {
                eprintln!("failed to load agent process: {}", error);
                std::process::exit(1);
            }

            if let Err(error) = process.start() {
                eprintln!("failed to start agent process: {}", error);
                std::process::exit(1);
            }

            let records = match run_fixed_steps(&mut process, 3) {
                Ok(records) => records,
                Err(error) => {
                    eprintln!("failed to run agent process: {:?}", error);
                    std::process::exit(1);
                }
            };

            println!(
                "loaded agent process: id={}, state={:?}, steps={}",
                process.id(),
                process.state(),
                records.len()
            );
        }
        Err(error) => {
            eprintln!("failed to load agent process: {:?}", error);
            std::process::exit(1);
        }
    }
}

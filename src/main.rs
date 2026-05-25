// App startup glue lives outside main so it can be tested.
pub mod app;
// Expose config loading as part of the engine surface.
pub mod config;
// Expose runtime process handling as part of the engine surface.
pub mod process;
// Expose the execution loop runtime.
pub mod runtime;
// Test-only helpers keep repeated setup out of production modules.
#[cfg(test)]
pub mod test_support;

// Later this will start the gRPC server and engine runtime.
fn main() {
    if let Err(error) = app::run_default_agent() {
        eprintln!("engine failed: {:?}", error);
        std::process::exit(1);
    }
}

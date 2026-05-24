// Expose config loading as part of the engine surface.
pub mod config;
// Expose runtime process handling as part of the engine surface.
pub mod process;

// Later this will start the gRPC server and engine runtime.
fn main() {
    println!("Hello, world!");
}

# Troubleshooting Entry 004: Rust Proto Field & Module Visibility

## The Problem (What)
When writing the live integration test client, we hit two distinct errors:
1. `error[E0560]`: `StartAgentRequest` has no field named `yaml_definition` (found `user_message`).
2. `error[E0432]`: Unresolved import `crate::runtime::GrpcToolClient`.

## The Cause (Why)
1. **Contract Drift**: The `agent.proto` file was updated to use `user_message` for a more flexible starting point, but our test code was still using an old version of the requirement (`yaml_definition`).
2. **Encapsulation**: `GrpcToolClient` was defined inside the `runtime::tool` submodule but wasn't "re-exported" in `runtime/mod.rs`, making it invisible to `main.rs`.

## The Fix (How)
1. **Field Alignment**: Updated `test_client.rs` to match the generated struct fields exactly (`user_message`).
2. **Re-exporting**: Added `pub use tool::GrpcToolClient;` to `engine/src/runtime/mod.rs`.
3. **Library Logic**: Created `engine/src/lib.rs` to expose the engine's modules so they could be accessed by the `bin/test_client.rs` as an external crate.

## Prevention
Always run `cargo check` after updating `.proto` files to ensure the generated Rust structs haven't drifted from your implementation logic. Use re-exports (`pub use`) to keep your public API clean while maintaining a deep folder structure.

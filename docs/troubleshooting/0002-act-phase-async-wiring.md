# Troubleshooting Entry 002: Rust ReAct Loop Async & Gateway Wiring

## The Problem (What)
When transforming the `act()` phase from a synchronous stub to a real gRPC-capable async function, the engine failed to compile with multiple errors across four files:
1. `error[E0061]`: `act` function takes 3 arguments but 2 were supplied.
2. `error[E0308]`: Expected `PhaseOutput`, found `Future` (mismatched types).
3. `error[E0277]`: `ToolGatewayError` doesn't implement `std::fmt::Display`.

## The Cause (Why)
This was a classic **Ripple Effect** caused by changing a core function signature in a strictly typed language:
1. **Dependency Injection**: The `Act` phase now requires a `ToolGateway` to perform its job, but the parent functions (`execute_step`, `run_agent_loop`) weren't passing it down.
2. **Async Migration**: Making a function `async` changes its return type from `T` to `Future<T>`. Every caller must now `.await` that function to get the actual value.
3. **Trait Requirements**: Using `format!("{}", err)` requires the error type to implement the `Display` trait, which was missing from our custom error enum.

## The Fix (How)
1. **Error Voice**: Added `thiserror` to `Cargo.toml` and used `#[error(...)]` macros in `error.rs` to satisfy the `Display` trait requirement.
2. **Generic Propagation**: Updated `execute_step` to be generic over `<G: ToolGateway>` so it can accept any gateway implementation (Mock or gRPC).
3. **Await Integration**: Added `.await` to the `act()` call inside `step_executor.rs` to resolve the future into a `PhaseOutput`.
4. **Shared Ownership**: Wrapped the gateway in an `Arc` (Atomically Reference Counted) pointer in `loop_runner.rs` and `AgentService` so multiple concurrent agents can safely share a single gateway connection.

## Prevention
When making a core phase `async`, immediately trace the call stack upwards. In Rust, `async` is "viral"—it usually requires the entire execution chain to become async as well.

# UAV Physics Simulator

A quadcopter simulation prototype with a Rust physics library and a C++ bridge for Unreal Engine. The standalone core implements propeller thrust, altitude-dependent air density, body-axis aerodynamic drag, a four-motor mixer and RK4 state integration.

## Engineering highlights

- Rust `cdylib`/`rlib` physics core with C ABI entry points.
- Generated C headers, interface-version checks and state-size checks.
- Position, velocity, quaternion orientation and angular-velocity state.
- Unreal plugin, drone Blueprint asset and telemetry component.
- Unit tests for air density, drag, throttle, free fall and hover equilibrium.

## Build the core

Install Rust through your preferred toolchain manager. The checked-in toolchain file selects the project version.

```sh
cd physics_core
cargo test --locked
cargo build --release --locked
```

The build generates `drone_ffi.h`. Synchronize it with the Unreal plugin's generated header before integration. The current build script does not copy the header automatically.

## Unreal integration

Open `UE5Project/UAV_Simulator.uproject` with a compatible Unreal installation; its engine association is 5.8. Build the Rust library first. The bridge build rules currently cover Windows and Linux. Windows also requires the appropriate import library alongside the DLL; validate its generation for your toolchain. macOS library linking is not implemented in those rules. Editor binaries and Rust build outputs are excluded.

## Verification and limits

All 13 existing Rust unit tests passed during portfolio preparation. Unreal compilation and editor operation were not verified. The model uses fixed vehicle parameters and simplified aerodynamics; it is not validated against flight measurements. This is a simulation prototype, not a real-flight controller.

The public repository is a clean source snapshot; the older private `UAV-Simulator` repository is retained separately.

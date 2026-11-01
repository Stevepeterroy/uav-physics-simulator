# UAV Physics Simulator

Collaborative team project originating in `elkwire/UAV-Simulator`. This copy does not imply sole authorship; see `COLLABORATION.md`.

A Rust quadcopter physics prototype with an Unreal Engine C++ bridge and an offline ONNX surrogate-model pipeline. This repository contains the current 0.3.0 core rather than the earlier standalone 0.1.0 snapshot.

## Engineering highlights

- Propeller thrust, altitude-dependent air density, body-axis drag and four-motor mixing.
- RK4 position, velocity and quaternion integration.
- Motor-temperature and structural-model experiments.
- C ABI entry points, generated headers, version/size guards and telemetry integration.
- ONNX model loading with canonical geometry-hash provenance checks.
- Offline Python generators for aerodynamic and structural surrogate interface examples.

## Build and test the core

```sh
cd physics_core
cargo test --locked
cargo build --release --locked
```

The checked-in Rust toolchain file selects the compiler. The `ort` dependency downloads ONNX Runtime binaries during setup and requires network access on a fresh build. Keep `offline_pipeline/geometry/current_geometry.json` available: the build hashes it and embeds the hash in the core. `build.rs` generates the Unreal bridge header under `Public/Generated/`.

## Optional offline surrogate examples

```sh
python3 -m venv .venv
.venv/bin/python -m pip install -r offline_pipeline/requirements.txt
.venv/bin/python offline_pipeline/cfd/train_aero.py
.venv/bin/python offline_pipeline/fea/train_fea.py
```

These scripts construct small ONNX graphs with fixed coefficients and metadata. They are interface demonstrations, not models trained on measured CFD/FEA data. A constant `in_validated_envelope` output does not establish physical validation. Generated model artifacts stay local.

## Unreal integration

Open `UE5Project/UAV_Simulator.uproject` with a compatible Unreal installation; its engine association is 5.8. Build the Rust library first. The bridge build rules currently cover Windows and Linux. Validate the Windows import-library step for the selected toolchain. macOS linking is not implemented in those build rules. Unreal editor compilation and runtime behavior are not verified by the standalone core checks.

## Status and limitations

The physics, thermal and structural parameters are simplified or placeholders and have not been validated against flight measurements. This is a research/simulation prototype, not a real-flight controller or a certified system. See `PUBLICATION.md` for checks performed on this update.

Do not commit credentials, private certificates, local datasets, generated model binaries or Unreal/Rust build outputs. The public portfolio copy is available at https://github.com/Stevepeterroy/uav-physics-simulator.

use physics_core::*;
use std::ptr::{null, null_mut};
#[test]
fn interface_layout_matches_rust_state() {
    assert_eq!(ffi_get_interface_version(), 1);
    assert_eq!(ffi_get_drone_state_size() as usize, std::mem::size_of::<types::DroneState>());
}
#[test]
fn null_arguments_fail_without_dereferencing() {
    unsafe {
        assert_eq!(ffi_reset_drone_state(null_mut()), 1);
        assert_eq!(ffi_get_debug_telemetry(null_mut()), 1);
        assert_eq!(ffi_step_physics(null_mut(), null(), null_mut(), null_mut(), 0.01), 1);
        assert_eq!(ffi_load_surrogate_model(null(), null_mut()), 1);
        assert_eq!(ffi_query_surrogate(null_mut(), null(), 0, null_mut()), 1);
        ffi_unload_surrogate_model(null_mut());
    }
}
#[test]
fn reset_restores_default_values() {
    let mut state = ffi_create_default_drone_state();
    state.position = [100.0, -20.0, 7.0];
    state.velocity = [1.0,2.0,3.0];
    unsafe { assert_eq!(ffi_reset_drone_state(&mut state),0); }
    let default = ffi_create_default_drone_state();
    assert_eq!(state.position, default.position);
    assert_eq!(state.velocity, default.velocity);
    assert_eq!(state.orientation, default.orientation);
}

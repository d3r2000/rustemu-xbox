pub mod xbox;

/// FFI proof-of-concept for the recomp-spiderman bridge.
/// Returns 42. If a C++ program can load rustemu_core.dll and call this
/// and get 42 back, the FFI boundary works and the recomp/rustemu hybrid
/// is feasible.
#[no_mangle]
pub extern "C" fn rust_hello() -> u32 {
    42
}

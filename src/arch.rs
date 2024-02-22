#[cfg(target_arch = "riscv64")]
mod riscv64gc;
#[cfg(target_arch = "riscv64")]
pub use riscv64gc::*;
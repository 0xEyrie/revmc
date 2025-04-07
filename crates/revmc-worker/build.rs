#![allow(missing_docs)]

fn main() {
    #[cfg(not(target_os = "macos"))]
    revmc_build::emit();
}

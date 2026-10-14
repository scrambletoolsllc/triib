//! Links CoreAudio weakly on macOS. The audio library's loopback recording
//! refers to Core Audio's process taps, which came with macOS 14.2, and a
//! strong reference would keep triib from starting on older systems at
//! all. Weakly linked, a function the system lacks is only missing where
//! it would be called; triib looks the taps up before it uses them.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-weak_framework,CoreAudio");
    }
}

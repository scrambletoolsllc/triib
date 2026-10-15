//! Prints what each interface does about EEE and PAUSE, and whether it
//! filters VLANs in hardware.

#[cfg(target_os = "linux")]
fn main() {
    for interface in avb_net::interfaces() {
        if let Ok(power) = avb_net::interfaces::link_power(&interface.name) {
            let vlans = avb_net::interfaces::filters_vlans(&interface.name);
            println!(
                "{}: EEE {}, PAUSE {}, VLAN filter {:?}",
                interface.name, power.eee, power.pause, vlans
            );
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("only on Linux");
}

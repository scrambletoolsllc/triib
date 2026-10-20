//! triib's helper for raw Ethernet, the one file granted CAP_NET_RAW
//! (`sudo setcap cap_net_raw+ep triib-netraw`): it opens packet sockets
//! for triib, triib-cli and triib-endpointd and passes them over, so they
//! hold no capability themselves and the desktop's portals still answer
//! triib.

#[cfg(target_os = "linux")]
fn main() {
    std::process::exit(avb_net::broker::serve());
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("triib-netraw is for Linux, which grants raw sockets by capability");
    std::process::exit(2);
}

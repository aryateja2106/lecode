//! `lesearch` — command-line client for the `LeSearch` daemon.
//!
//! Phase A.0 stub. Real commands (`run`, `ls`, `logs`, `sessions`, `daemon`)
//! ship with the protocol implementation in Day 2-4.

fn main() {
    tracing_subscriber::fmt().init();
    println!("lesearch {} — scaffold only", lesearch_protocol::version());
    println!("run `lesearch --help` once Day 2 protocol implementation lands");
}

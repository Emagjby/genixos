pub mod serial;

pub fn init() {
    serial::with_serial1(|s| s.init());
}

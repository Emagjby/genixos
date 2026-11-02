use crate::arch::x86_64::interrupts;
use crate::time::duration::Duration;

pub fn uptime() -> Duration {
    let ns = interrupts::get_current_tick().saturating_mul(10_000_000);
    Duration::from_nanos(ns)
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy)]
pub struct Duration {
    nanoseconds: u64,
}

impl Duration {
    pub const ZERO: Self = Self { nanoseconds: 0 };

    pub const fn from_nanos(ns: u64) -> Duration {
        Self { nanoseconds: ns }
    }

    pub const fn from_millis(ms: u64) -> Option<Duration> {
        let ns = ms.checked_mul(1_000_000);

        if let Some(ns) = ns {
            Some(Self { nanoseconds: ns })
        } else {
            None
        }
    }

    pub const fn from_secs(s: u64) -> Option<Duration> {
        let ns = s.checked_mul(1_000_000_000);

        if let Some(ns) = ns {
            Some(Self { nanoseconds: ns })
        } else {
            None
        }
    }

    pub const fn as_nanos(&self) -> u64 {
        self.nanoseconds
    }

    pub const fn as_millis(&self) -> u64 {
        self.nanoseconds / 1_000_000
    }

    pub const fn as_secs(&self) -> u64 {
        self.nanoseconds / 1_000_000_000
    }
}

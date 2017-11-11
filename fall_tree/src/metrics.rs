use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::fmt;

pub struct Metrics {
    metrics: Mutex<Vec<Metric>>,
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for m in self.metrics.lock().unwrap().iter() {
            write!(f, "{}\n", m)?;
        }
        Ok(())
    }
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics { metrics: Default::default() }
    }

    pub fn record(&self, name: &'static str, value: u64, unit: &'static str) {
        self.metrics.lock().unwrap().push(Metric { name, value, unit })
    }

    pub fn record_time(&self, name: &'static str, value: Duration) {
        let value = (value.subsec_nanos() as u64) / 1_000_000 + value.as_secs() * 1_000;
        self.record(name, value, "ms")
    }

    pub fn measure_time<T, F: FnOnce() -> T, >(&self, name: &'static str, f: F) -> T {
        let instant = Instant::now();
        let result = f();
        self.record_time(name, instant.elapsed());
        result
    }

    pub fn get(&self, name: &'static str) -> Option<u64> {
        self.metrics.lock().unwrap().iter().find(|m| m.name == name).map(|m| m.value)
    }
}

pub struct Metric {
    pub name: &'static str,
    pub value: u64,
    pub unit: &'static str,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {}", self.name, self.value, self.unit)
    }
}

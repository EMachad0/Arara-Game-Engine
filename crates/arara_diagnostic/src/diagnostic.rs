use arara_time::{Duration, Instant};
use arara_utils::StableHashMap;
use std::collections::VecDeque;

/// A single measurement of a [Diagnostic]
#[derive(Debug)]
pub struct DiagnosticMeasurement {
    pub time: Instant,
    pub value: f64,
}

/// A timeline of [DiagnosticMeasurement]s of a specific type.
/// Diagnostic examples: frames per second, CPU usage, network latency
#[derive(Debug)]
pub struct Diagnostic {
    pub name: &'static str,
    history: VecDeque<DiagnosticMeasurement>,
    sum: f64,
    max_history_length: usize,
}

impl Diagnostic {
    pub fn add_measurement(&mut self, value: f64) {
        let time = Instant::now();
        if self.history.len() == self.max_history_length {
            if let Some(removed_diagnostic) = self.history.pop_back() {
                self.sum -= removed_diagnostic.value;
            }
        }

        self.sum += value;
        self.history
            .push_front(DiagnosticMeasurement { time, value });
    }

    pub fn new(name: &'static str, max_history_length: usize) -> Diagnostic {
        Diagnostic {
            name,
            history: VecDeque::with_capacity(max_history_length),
            max_history_length,
            sum: 0.0,
        }
    }

    pub fn value(&self) -> Option<f64> {
        self.history.back().map(|measurement| measurement.value)
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn average(&self) -> Option<f64> {
        if !self.history.is_empty() {
            Some(self.sum / self.history.len() as f64)
        } else {
            None
        }
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn duration(&self) -> Option<Duration> {
        if self.history.len() < 2 {
            return None;
        }

        if let Some(oldest) = self.history.back() {
            if let Some(newest) = self.history.front() {
                return Some(newest.time.duration_since(oldest.time));
            }
        }

        None
    }

    pub fn get_max_history_length(&self) -> usize {
        self.max_history_length
    }
}

/// A collection of [Diagnostic]s
#[derive(Debug, Default)]
pub struct Diagnostics {
    // This uses a [`StableHashMap`] to ensure that the iteration order is deterministic between
    // runs when all diagnostics are inserted in the same order.
    diagnostics: StableHashMap<&'static str, Diagnostic>,
}

impl Diagnostics {
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.insert(diagnostic.name, diagnostic);
    }

    pub fn get(&self, name: &str) -> Option<&Diagnostic> {
        self.diagnostics.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Diagnostic> {
        self.diagnostics.get_mut(name)
    }

    pub fn get_measurement(&self, name: &str) -> Option<&DiagnosticMeasurement> {
        self.diagnostics
            .get(name)
            .and_then(|diagnostic| diagnostic.history.front())
    }

    pub fn add_measurement(&mut self, name: &str, value: f64) {
        if let Some(diagnostic) = self.diagnostics.get_mut(name) {
            diagnostic.add_measurement(value);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.values()
    }
}

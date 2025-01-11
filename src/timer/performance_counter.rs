// Copyright (c) 2025 Lucas B. Andrade
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::ops::{Add, Sub};

use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

static mut FREQUENCY: u64 = 0;

/// Represents a performance counter that can be used to measure time.
/// Make sure to call `PerformanceCounter::init()` before using the performance counter.
/// 
/// # Example
/// ```
/// use sky_labs::timer::PerformanceCounter;
/// 
/// PerformanceCounter::init();
/// let start = PerformanceCounter::now();
/// // Do something
/// let end = PerformanceCounter::now();
/// println!("Elapsed time: {} seconds", (end - start).total_seconds());
/// ```
/// # Notes
/// The performance counter is based on the Windows API QueryPerformanceCounter and QueryPerformanceFrequency.
/// The performance counter is not thread-safe.
/// The performance counter should not be used to display the current time to the user.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct PerformanceCounter {
    pub(super) ticks: u64,
}

impl PerformanceCounter {
    /// Initializes the performance counter module. Must be called before using the performance counter.
    pub fn init() {
        unsafe {
            let mut frequency = 0i64;
            QueryPerformanceFrequency(&mut frequency).unwrap();
            FREQUENCY = frequency as u64;
        }
    }

    /// Creates a new performance counter with zero ticks.
    pub fn new() -> Self {
        PerformanceCounter { ticks: 0 }
    }

    /// Creates a new performance counter with the current time.
    pub fn now() -> Self {
        let mut qpc: i64 = 0;
        unsafe {
            QueryPerformanceCounter(&mut qpc).unwrap();
        }
        PerformanceCounter { ticks: qpc as u64 }
    }

    /// Returns the frequency of the performance counter
    pub fn frequency() -> u64 {
        unsafe { FREQUENCY }
    }

    /// Returns total seconds passed by the performance counter
    pub fn total_seconds(&self) -> f64 {
        unsafe {
            debug_assert!(FREQUENCY != 0, "PerformanceCounter::init() must be called before using the performance counter.");
            self.ticks as f64 / FREQUENCY as f64
        }
    }

    /// Returns the seconds component of the performance counter
    pub fn seconds(&self) -> u64 {
        unsafe {
            debug_assert!(FREQUENCY != 0, "PerformanceCounter::init() must be called before using the performance counter.");
            self.ticks % FREQUENCY
        }
    }

    /// Returns total milliseconds passed by the performance counter
    pub fn total_milliseconds(&self) -> f64 {
        unsafe {
            debug_assert!(FREQUENCY != 0, "PerformanceCounter::init() must be called before using the performance counter.");
            (self.ticks as f64 * 1000f64) / FREQUENCY as f64
        }
    }

    /// Returns the milliseconds component of the performance counter
    pub fn milliseconds(&self) -> u64 {
        unsafe {
            debug_assert!(FREQUENCY != 0, "PerformanceCounter::init() must be called before using the performance counter.");
            self.ticks % (FREQUENCY * 1000)
        }
    }
}

impl Add for PerformanceCounter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        PerformanceCounter {
            ticks: self.ticks + rhs.ticks,
        }
    }
}

impl Sub for PerformanceCounter {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        PerformanceCounter {
            ticks: self.ticks - rhs.ticks,
        }
    }
}

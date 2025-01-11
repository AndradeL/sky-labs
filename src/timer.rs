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

pub mod framerate_counter;
pub mod performance_counter;

pub use self::{framerate_counter::FramerateCounter, performance_counter::PerformanceCounter};

/// A timer that can be used to measure time between frames.
/// Call `tick` to update the timer and call the update function at the start of each frame.
/// 
/// # Example
/// ```
/// use sky_labs::timer::StepTimer;
/// 
/// let mut timer = StepTimer::new();
/// loop {
///     timer = timer.tick(|timer| {
///         println!("Elapsed time: {} seconds", timer.elapsed_seconds());
///         // Do something
///     });
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepTimer {
    pub current_time: PerformanceCounter,
    pub last_time: PerformanceCounter,
}

impl StepTimer {
    /// Creates a new StepTimer.
    pub fn new() -> Self {
        PerformanceCounter::init();
        StepTimer {
            current_time: PerformanceCounter::default(),
            last_time: PerformanceCounter::default(),
        }
    }

    /// Updates the timer and calls the update function.
    pub fn tick<F>(&self, f_update: F) -> Self
    where
        F: Fn(&Self) -> (),
    {
        let now = PerformanceCounter::now();
        let new_timer = StepTimer {
            current_time: now,
            last_time: self.current_time,
        };
        f_update(&new_timer);
        new_timer
    }

    /// Returns the time elapsed since the last tick.
    pub fn elapsed(&self) -> PerformanceCounter {
        self.current_time - self.last_time
    }

    /// Returns the time elapsed since the last tick in seconds.
    pub fn elapsed_seconds(&self) -> f64 {
        (self.current_time - self.last_time).total_seconds()
    }

    /// Returns the time elapsed since the last tick in milliseconds.
    pub fn elapsed_ms(&self) -> f64 {
        (self.current_time - self.last_time).total_milliseconds()
    }
}

use std::time;

/// Timer in miliseconds (ms). Valid only for ~500,000,000 years.
pub struct Timer {
    /// Time of instantiation
    instant: time::Instant,
    /// The time as of last check
    previous_time: u64,
    /// Total time elapsed since starting the timer (while unpaused)
    pub elapsed: u64,
    /// Whether or not the timer is paused
    pub paused: bool,
}

impl Timer {
    /// Create a `Timer`. Paused by default
    pub fn new() -> Self {
        Timer {
            instant: time::Instant::now(),
            previous_time: 0,
            elapsed: 0,
            paused: true,
        }
    }

    /// Begin keeping track of time
    pub fn start(&mut self) {
        self.previous_time = self.instant.elapsed().as_millis() as u64;
        self.paused = false;
    }

    /// Pause the timer (does nothing if already paused)
    pub fn pause(&mut self) {
        self.paused = true;
    }
    
    /// Unpause the timer (does nothing if unpaused)
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Updates `elapsed` with time since last call. Returns this value (delta time).
    pub fn tick(&mut self) -> u64 {
        let elapsed = self.instant.elapsed().as_millis() as u64;

        let delta_time = elapsed - self.previous_time;
        self.previous_time = elapsed;

        if !self.paused {
            self.elapsed += delta_time;
        }

        delta_time
    }

    /// Sets `elapsed` time to 0
    pub fn reset_elapsed(&mut self) {
        self.elapsed = 0;
    }

    /// Locks thread for the remainder of one frame (if applicable)
    /// - `target_fps`: Desired frames per second
    /// - `delta_time`: Time since last frame in ms (returned by `Timer::tick()`)
    /// - `threshold`: Threshold in ms for skipping sleep (e.g.: don't sleep for less than 2 ms)
    // NOTE: This is a static method to force the user to call timer.tick() explicitly
    pub fn await_fps(target_fps: u64, delta_time: u64, threshold: u64) {
        let ms_per_frame = 1000 / target_fps;
        // println!("delta_time: {}", delta_time);
        if (delta_time < ms_per_frame) && (ms_per_frame - delta_time > threshold) {
            // println!("Sleeping for {} ms", ms_per_frame - delta_time);
            std::thread::sleep(
                time::Duration::from_millis(ms_per_frame - delta_time)
            );
        }
    }
}
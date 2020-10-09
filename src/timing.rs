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

    /// Locks the thread until the `target_fps` is met based on the last call to `tick()`. 
    ///   
    /// Does nothing if more time has elapsed since the last call to `tick()` than desired fps as ms per frame.
    /// - `threshold` determines whether to skip sleeping if the desired time per frame is close to the time since last `tick()`
    // NOTE: Sleep for one less ms since timing is not guarenteed to be exact.
    pub fn await_fps(&mut self, target_fps: u64, threshold: u64) {
        let ms_per_frame = 1000 / target_fps;
        let elapsed = self.instant.elapsed().as_millis() as u64;
        let dt = elapsed - self.previous_time;
        
        // This is actually the time per frame
        // println!("sleep dt: {}", dt); 

        if (dt < ms_per_frame) && (ms_per_frame - dt > threshold) {
            // println!("Sleeping for {} ms", ms_per_frame - dt - 1);
            std::thread::sleep(
                time::Duration::from_millis(ms_per_frame - dt - 1)
            );
        }
    }
}
/*
    This file is taken from my WIP Engine
*/

// Using sdl2 for timing is "not recommended" (but it is easy to use)
// TODO: Rewrite this using the std::time API


/// Timer in miliseconds (ms)
pub struct Timer {
    /// The time as of last check
    previous_time: u32,
    /// Total time elapsed since starting the timer (while unpaused)
    pub elapsed: u32,
    /// Whether or not the timer is paused
    pub paused: bool,
    
    timer: sdl2::TimerSubsystem,
}

impl Timer {
    /// Create a `Timer`. Paused by default
    pub fn new(timer: sdl2::TimerSubsystem) -> Self {
        Timer {
            previous_time: 0,
            elapsed: 0,
            paused: true,
            timer,
        }
    }

    /// Create `Timer` from an sdl2 context. Paused by default
    pub fn from_sdl2_context(context: &sdl2::Sdl) -> Self {
        Self::new(context.timer().expect("Failed to init timer subsystem"))
    }

    /// Begin keeping track of time
    pub fn start(&mut self) {
        self.previous_time = self.timer.ticks();
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

    /// Toggles the timer being paused
    /// 
    /// Returns `true` if this call pauses or `false` if unpauses.
    pub fn toggle_paused(&mut self) -> bool {
        if self.paused {
            self.resume();
            false
        } else {
            self.pause();
            true
        }
    }

    /// Updates `elapsed` with time since last call. Returns this value (delta time).
    pub fn tick(&mut self) -> u32 {
        let ticks = self.timer.ticks();
        let delta_time = ticks - self.previous_time;
        self.previous_time = ticks;

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
    /// - `target_fps`: Frames per second in ms
    /// - `delta_time`: Time since last frame in ms
    /// - `threshold`: Threshold in ms for skipping sleep
    // NOTE: This is a static method to force the user to call timer.tick() explicitly
    pub fn await_fps(target_fps: u32, delta_time: u32, threshold: u32) {
        let ms_per_frame = 1000 / target_fps;
        // println!("delta_time: {}", delta_time);
        if (delta_time < ms_per_frame) && (ms_per_frame - delta_time > threshold) {
            // println!("Sleeping for {} ms", ms_per_frame - delta_time);
            std::thread::sleep(
                std::time::Duration::new(0, (ms_per_frame - delta_time) * 1_000_000)
            );
        }
    }
}
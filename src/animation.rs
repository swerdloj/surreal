pub enum AnimationStatus {
    InProgress,
    Complete,
}

pub struct Animation<T> {
    animation: Box<dyn Fn(&mut T, u32)>,
    duration: u32,
    elapsed: u32,
}

impl<T> Animation<T> {
    pub fn new<F: Fn(&mut T, u32) + 'static>(animation: F, duration: u32) -> Self {
        Self {
            animation: Box::new(animation),
            duration,
            elapsed: 0,
        }
    }

    pub fn animate(&mut self, target: &mut T, dt: u32) -> AnimationStatus {
        self.elapsed += dt;

        if self.elapsed <= self.duration {
            (self.animation)(target, self.elapsed);

            AnimationStatus::InProgress
        } else {
            AnimationStatus::Complete
        }
    }
}
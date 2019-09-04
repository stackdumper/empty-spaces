/**
 * Clock resource
 * provides information about how much time elapsed
 * since the previous game step. Allows to keep game speed
 * independent from game FPS.
 */
#[derive(Default)]
pub struct Clock {
    pub dt: f64,
}

impl Clock {
    pub fn new(dt: f64) -> Self {
        Self { dt }
    }
}

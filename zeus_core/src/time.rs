use std::time::Instant;

pub struct Stopwatch {
    last_frame_time: Instant,
    delta: u128,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            last_frame_time: Instant::now(),
            delta: 0,
        }
    }

    pub fn update_time(&mut self) {
        self.delta = self.last_frame_time.elapsed().as_millis();
        self.last_frame_time = Instant::now();
    }

    pub fn get_current_delta(&self) -> u128 {
        self.last_frame_time.elapsed().as_millis()
    }

    pub fn get_delta(&self) -> u128 {
        self.delta
    }

    pub fn get_delta_seconds(&self) -> u128 {
        self.delta / 1000
    }

    pub fn get_delta_f64(&self) -> f64 {
        self.delta as f64
    }

    pub fn get_delta_f32(&self) -> f32 {
        self.delta as f32
    }

    pub fn get_delta_seconds_f64(&self) -> f64 {
        (self.delta as f64) / 1000.0
    }

    pub fn get_delta_seconds_f32(&self) -> f32 {
        (self.delta as f32) / 1000.0
    }

    pub fn get_framerate(&self) -> f32 {
        if self.delta == 0 {
            return 0.0;
        }

        1000.0 / (self.delta as f32)
    }

    pub fn check_delta(&self) -> u128 {
        self.last_frame_time.elapsed().as_millis()
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn get_delta_f64() {
        let mut sw = super::Stopwatch::new();

        sleep(Duration::new(0, 500_000_000));

        sw.update_time();

        assert!(sw.get_delta_f64() >= 500.0);
    }
}

use std::time::{ Duration, Instant };

pub struct FrameCount {
    count: usize,
    count_in_duration: usize,
    last_fps: usize,
    start_time: Instant,
    next_time: Instant,
    enabled_log: bool,
}

impl FrameCount {
    pub fn new() -> FrameCount {
        let now = Instant::now();
        FrameCount {
            count: 0,
            count_in_duration: 0,
            last_fps: 0,
            start_time: now,
            next_time: now,
            enabled_log: false
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
        self.count_in_duration += 1;
        let now = Instant::now();
        if now >= self.next_time {
            self.last_fps = self.count_in_duration;
            self.count_in_duration = 0;
            self.next_time = now + Duration::from_secs(1);
            if self.enabled_log {
                self.log();
            }
        }
    }

    fn log(&self) {
        println!("fps: {}", self.last_fps);
    }

    pub fn toggle_log(&mut self, b: bool) {
        self.enabled_log = b;
    }
}
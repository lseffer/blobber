use femtovg::FontId;
use femtovg::{Align, Baseline, Canvas, Color, Paint, Renderer};
use std::time::Instant;

pub struct Fonts {
    pub regular: FontId,
}

struct WatchHistory {
    history_count: usize,
    values: Vec<f32>,
    head: usize,
}

impl WatchHistory {
    pub fn new(size: usize) -> Self {
        WatchHistory {
            history_count: size,
            values: vec![0.0; size],
            head: Default::default(),
        }
    }

    fn get_average(&self) -> f32 {
        self.values.iter().map(|v| *v).sum::<f32>() / self.history_count as f32
    }

    fn update(&mut self, val: f32) {
        self.head = (self.head + 1) % self.history_count;
        self.values[self.head] = val;
    }
}

pub struct StopWatch {
    start_time: Instant,
    ups_counter: u64,
    fps_history: WatchHistory,
    ups_history: WatchHistory,
}

impl StopWatch {
    pub fn new(size: usize) -> Self {
        StopWatch {
            fps_history: WatchHistory::new(size),
            ups_history: WatchHistory::new(size),
            start_time: Instant::now(),
            ups_counter: 0,
        }
    }

    fn get_fps(&self, instant: Instant) -> f32 {
        let frame_time = (instant - self.start_time).as_secs_f32();
        return 1.0 / frame_time;
    }

    fn get_ups(&self, instant: Instant) -> f32 {
        let diff = instant - self.start_time;
        return self.ups_counter as f32 / diff.as_secs_f32();
    }

    pub fn simulate(&mut self) {
        self.ups_counter += 1;
    }

    fn update_values(&mut self, now: Instant) {
        self.ups_history.update(self.get_ups(now));
        self.fps_history.update(self.get_fps(now));
        self.start_time = now;
        self.ups_counter = 0;
    }

    pub fn render<T: Renderer>(&mut self, canvas: &mut Canvas<T>, fonts: &Fonts) {
        self.update_values(Instant::now());
        let text = format!(
            "FPS: {:.2}, UPS: {:.2}",
            self.fps_history.get_average(),
            self.ups_history.get_average(),
        );
        let mut paint = Paint::color(Color::rgba(0, 0, 0, 255));
        paint.set_font_size(14.0);
        paint.set_font(&[fonts.regular]);
        paint.set_text_align(Align::Left);
        paint.set_text_baseline(Baseline::Top);
        let _ = canvas.fill_text(10.0, 10.0, text, paint);
    }
}

#[cfg(test)]
mod watch_history_tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut wh = WatchHistory::new(10);
        wh.update(1.0);
        assert!(wh.values.len() == 10);
        assert!(wh.values[1] == 1.0);
    }

    #[test]
    fn test_update_overflow() {
        let mut wh = WatchHistory::new(2);
        wh.update(1.0);
        wh.update(2.0);
        wh.update(3.0);
        assert!(wh.values.len() == 2);
        assert!(wh.values[0] == 2.0);
        assert!(wh.values[1] == 3.0);
    }

    #[test]
    fn test_average() {
        let mut wh = WatchHistory::new(2);
        assert!(wh.get_average() == 0.0);
        wh.update(1.0);
        wh.update(2.0);
        assert!(wh.get_average() == 1.5);
    }
}

#[cfg(test)]
mod stop_watch_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_simulate() {
        let mut sw = StopWatch::new(2);
        sw.simulate();
        assert!(sw.ups_counter == 1);
    }
    #[test]
    fn test_get_fps() {
        let now = Instant::now();
        let then = now + Duration::from_secs(2);
        let mut sw = StopWatch::new(2);
        sw.start_time = now;
        let result = sw.get_fps(then);
        assert!(result == 0.5);
    }

    #[test]
    fn test_get_ups() {
        let now = Instant::now();
        let then = now + Duration::from_secs(2);
        let mut sw = StopWatch::new(2);
        sw.start_time = now;
        sw.ups_counter = 2;
        let result = sw.get_ups(then);
        assert!(result == 1.0);
    }

    #[test]
    fn test_update_values() {
        let now = Instant::now();
        let then = now + Duration::from_secs(2);
        let mut sw = StopWatch::new(2);
        sw.start_time = now;
        sw.ups_counter = 2;
        sw.update_values(then);
        assert!(sw.fps_history.values[1] == 0.5);
        assert!(sw.ups_history.values[1] == 1.0);
    }
}

use std::time::{Duration, Instant};

pub struct Metrics {
    pub enabled: bool,
    pub total: Duration,
    pub parsing: Option<Duration>,
    pub part1: Option<Duration>,
    pub part2: Option<Duration>,
}

impl Metrics {
    pub fn start(enabled: bool) -> Self {
        Self {
            enabled,
            total: Duration::ZERO,
            parsing: None,
            part1: None,
            part2: None,
        }
    }

    pub fn track_parsing<TFn, R>(&mut self, f: TFn) -> R
    where
        TFn: FnOnce() -> R,
    {
        if !self.enabled {
            return f();
        }

        let start = Instant::now();
        let result = f();
        self.parsing = Some(start.elapsed());
        result
    }

    pub fn track_part1<TFn, R>(&mut self, f: TFn) -> R
    where
        TFn: FnOnce() -> R,
    {
        if !self.enabled {
            return f();
        }

        let start = Instant::now();
        let result = f();
        self.part1 = Some(start.elapsed());
        result
    }

    pub fn track_part2<TFn, R>(&mut self, f: TFn) -> R
    where
        TFn: FnOnce() -> R,
    {
        if !self.enabled {
            return f();
        }

        let start = Instant::now();
        let result = f();
        self.part2 = Some(start.elapsed());
        result
    }

    pub fn finish(mut self) -> Self {
        if !self.enabled {
            return self;
        }

        self.total = self.parsing.unwrap_or_default()
            + self.part1.unwrap_or_default()
            + self.part2.unwrap_or_default();

        self
    }
}

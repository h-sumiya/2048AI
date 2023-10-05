pub struct TimeManager {
    start: std::time::Instant,
    first: bool,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
            first: true,
        }
    }

    pub fn ok(&self) -> bool {
        let ep = self.start.elapsed().as_millis();
        if self.first {
            return ep < 990;
        } else {
            return ep < 40;
        }
    }

    pub fn next(&mut self) {
        let mut buf = String::new();
        for _ in 0..6 {
            std::io::stdin().read_line(&mut buf).unwrap();
            buf.clear();
        }
        self.start = std::time::Instant::now();
        self.first = false;
    }
}

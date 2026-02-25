pub struct LineEntry {
    pub line: usize,
    pub count: usize,
}

pub struct LineTracker {
    entries: Vec<LineEntry>,
}

impl LineTracker {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn push(&mut self, line: usize) {
        if let Some(last) = self.entries.last_mut().filter(|last| last.line == line) {
            last.count += 1;
            return;
        }

        self.entries.push(LineEntry { line, count: 1 });
    }

    pub fn line_at(&self, offset: usize) -> Option<usize> {
        let mut acc = 0usize;
        for e in &self.entries {
            acc += e.count;
            if offset < acc {
                return Some(e.line);
            }
        }
        None
    }
}

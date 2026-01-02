#[derive(Debug, Clone, PartialEq)]
pub struct Timer {
    duration: Option<u32>,
    counter: u32,
    is_active: bool,
}

impl Timer {
    pub fn new(duration: Option<u32>) -> Self {
        Self {
            duration,
            counter: 0,
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.counter = 0;
    }

    pub fn increment(&mut self) {
        if self.is_active {
            self.counter = self.counter.saturating_add(1);
            if let Some(duration) = self.duration
                && self.counter >= duration
            {
                self.is_active = false;
                self.counter = 0;
            }
        }
    }

    pub fn reset(&mut self) {
        self.counter = 0;
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn counter(&self) -> u32 {
        self.counter
    }
}

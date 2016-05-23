use std::collections::HashSet;

struct Watcher<T> {
    value: T,
    callbacks: HashSet<fn()>
}

impl<T> Watcher<T> {
    pub fn new(value: T) -> Watcher<T> {
        Watcher {
            value: value,
            callbacks: HashSet::new()
        }
    }

    pub fn register_callback(&mut self, f: fn()) -> bool {
        self.callbacks.insert(f)
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        for callback in &self.callbacks {
            callback();
        }
    }
}

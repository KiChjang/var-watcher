use std::collections::HashSet;

pub struct Watcher<T> {
    value: T,
    callbacks: HashSet<fn(*const T)>
}

impl<T> Watcher<T> {
    pub fn new(value: T) -> Watcher<T> {
        Watcher {
            value: value,
            callbacks: HashSet::new()
        }
    }

    pub fn register_callback(&mut self, f: fn(*const T)) -> bool {
        self.callbacks.insert(f)
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        for callback in &self.callbacks {
            callback(&self.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiate_callback() {
        fn callback(val: *const u8) {
            unsafe { assert_eq!(*val, 7); }
        }
        let mut watcher = Watcher::new(5);
        watcher.register_callback(callback);

        watcher.set(7);
    }
}

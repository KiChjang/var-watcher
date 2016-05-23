use std::collections::HashSet;

struct Watcher<T> {
    value: T,
    callbacks: HashSet<fn(T)>
}

impl<T> Watcher<T> {
    pub fn new(value: T) -> Watcher<T> {
        Watcher {
            value: value,
            callbacks: HashSet::new()
        }
    }
}

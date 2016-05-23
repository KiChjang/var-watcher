use std::collections::HashSet;

struct Watcher<T> {
    inner: T,
    callbacks: HashSet<fn(T)>
}

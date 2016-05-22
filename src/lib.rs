use std::boxed::Box;
use std::collections::HashSet;
use std::ops::Fn;

struct Watcher<T> {
    inner: T,
    callbacks: HashSet<Box<Fn(T)>>
}

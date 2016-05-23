# Variable Watcher

Variable Watcher is a Rust crate that provides a wrapper around a variable, and calls all registered callback functions when the variable is mutated.

## Future improvements
* Generalize all sorts of function pointers stored by using the std::ops::Fn trait
* Make it possible for callbacks to have arguments

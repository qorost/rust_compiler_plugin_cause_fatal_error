# Rust Compilers Plugins and Macros Conflicts


This code is originally to illustrate the conflicts, posted on [forum page](https://users.rust-lang.org/t/macros-and-compiler-plugin-conflicts/10328).

However when this code generates a `rustc overflow` when compile it, and this errors disappear, when you use it on single item. 

```
thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: Could not compile `macro_compiler_conflicts`.
```

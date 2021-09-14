# Autoincrement

It is small library for implementing autoincremental values, such as IDs or others.

```toml
[dependencies]
autoincrement = "1"
```

## Synchronous generator example

```rust
use autoincrement::prelude::*;

#[derive(Incremental, PartialEq, Eq, Debug)]
struct MyId(u32);

let mut generator = MyId::init();

assert_eq!(generator.pull(), MyId(1));
assert_eq!(generator.pull(), MyId(2));
assert_eq!(generator.pull(), MyId(3));
```

## Set first value

```rust
use autoincrement::prelude::*;

#[derive(Incremental, PartialEq, Eq, Debug)]
struct MyId(u32);

let mut generator = MyId(20).init_from();

assert_eq!(generator.pull(), MyId(20));
assert_eq!(generator.pull(), MyId(21));
assert_eq!(generator.pull(), MyId(22));
```

## Example with using thread-safe generator

```toml
[dependencies]
autoincrement = { version = "1", features = ["derive", "async"] }
```

```rust
use autoincrement::prelude::*;

#[derive(AsyncIncremental, PartialEq, Eq, Debug)]
struct MyId(u32);

let generator = MyId::init(); // does not need to be mutable

assert_eq!(generator.pull(), MyId(1));
assert_eq!(generator.pull(), MyId(2));
assert_eq!(generator.pull(), MyId(3));
```
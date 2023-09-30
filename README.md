# `eitherable`

## Info

This crate extends `bool` with a simple way to create the `Either` type, from
the `either` crate.

It is a simple convenience trait and method, and also a way for me to learn
about how crates.io and cargo work.

### Why is this not in the `either` crate

The `either` crate emphasies that the two sides of `Either`, `Left` nd `Right`,
have no element of truthiness or falsiness. However, the trait here immediately
assigns `Left` to be the truthy value. So this is not quite suitable for the
`either` crate.

The reason I chose `Left` to be the truthy value is simple: it retains the dame
order as the `if` statement. The two following statements are equivalent.

```rust
let fst_example = if my_cond { Either::Left(left) } else { Either::Right(right) };
let snd_example = my_cond.either(left, right);
```

If you think `Right` should be the truthy value, you can `.flip()`.

## Usage

### Dependency

Add the library as a dependency to your project by inserting

```toml
eitherable = "0.1.0"
```

into the `[dependencies]` section of your Cargo.toml file.

### Example

```rust
use eitherable::*;
    
let x = true;
assert_eq!(x.either(1, "right"), Either::Left(1));
    
let x = false;
assert_eq!(x.either(1, "right"), Either::Right("right"));

let x = true;
assert_eq!(x.either_else(|| 1,|| "right"), Either::Left(1));

let x = false;
assert_eq!(x.either_else(|| 1,|| "right"), Either::Right("right"));
```

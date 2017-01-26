# splice

~~[Documentation](https://docs.rs/splice)~~ (not until i push this, which might not happen)

As of Rust 1.14, there are a missing set of functions on the Vec struct. There are methods to insert
a single item to the middle of a Vec, and there are methods to append a Vec onto another Vec, but
there is nothing to take a slice or Vec and insert it into the middle of another Vec.

Enter `splice`:

```rust
let mut dest: Vec<String> = vec!["one".into(), "two".into()];
let src: Vec<String> = vec!["three".into(), "four".into()];

splice::splice_clone(&mut dest, 1, &src);

assert_eq!(dest, vec!["one", "three", "four", "two"]);
```

`splice` introduces three methods to insert a collection of values into the middle of a Vec:

* `splice_clone` which clones every element of the given slice,
* `splice_copy` which copies every element of a given slice, and
* `splice` which moves every element of the given Vec (and empties it).

To import this crate, add the following to your Cargo.toml:

```toml
[dependencies]
splice = "1.0.0"
```

...and the following to your crate root:

```rust
extern crate splice;
```

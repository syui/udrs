the cli tool of the url decode written in rust.

```sh
$ cargo test
$ cargo run "foo%20bar"
$ cargo build
$ ./target/debug/udrs

$ udrs --help
$ udrs "foo%20bar"
```

### v 0.1.1

```sh
$ echo "foo%20bar" | udrs
```

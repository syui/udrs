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
# pipe
$ echo "foo%20bar" | udrs
```

### v 0.1.2

```sh
# debug
$ RUST_LOG=udrs=debug ./udrs "foo%20bar" 

# makefile
$ make run
$ make install
```


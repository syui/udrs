the cli tool of the base64 encode,decode written in rust.

```sh
$ cargo run e "hello world."
aGVsbG8gd29ybGQu

$ cargo run d "aGVsbG8gd29ybGQu"
hello world.
```

### v0.1.1

```sh
# pipe
$ echo "foo%20bar" | udrs
```

### v0.1.2

```sh
# debug
$ RUST_LOG=udrs=debug ./udrs "foo%20bar" 

# makefile
$ make run
$ make install
```

### v0.1.3

```sh
#base64 encode
$ udrs b64e "hello"
```

### v0.1.4

```sh
#base64 encode
$ udrs e "hello world."
aGVsbG8gd29ybGQu

$ udrs d "aGVsbG8gd29ybGQu"
hello world.

$ udrs ud "foo%20bar"
foo bar
```

### v0.1.5

```sh
$ udrs ud "https://github.com/ksk001100/seahorse" -l
ksk001100/seahorse

$ udrs ud "https://github.com/ksk001100/seahorse" -p
https

$ udrs ud "https://github.com/ksk001100/seahorse" -d
github.com
```

### v0.1.6

```sh
# domain -> ip address (v4)
$ udrs ud "https://github.com" -i
V4(52.192.72.89:443)
```


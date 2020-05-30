the cli tool of the base64 encode,decode written in rust.

```sh
$ cargo run e "hello world."
aGVsbG8gd29ybGQu

$ cargo run d "aGVsbG8gd29ybGQu"
hello world.
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

$ dig github.com
52.192.72.89
```

### v0.1.7

```
seahorse "0.7.1" -> "1.0.0"
```


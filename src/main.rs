use std::env;
use seahorse::{App, Command, Context};
use url::percent_encoding::percent_decode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli_tool [command] [x] [y]")
        .command(
            Command::new()
                .name("b64e")
                .usage("udrs b64e {}")
                .action(b64e),
        )
        .command(
            Command::new()
                .name("b64d")
                .usage("udrs b64d {}")
                .action(b64d),
        )
        .command(
            Command::new()
                .name("d")
                .usage("udrs d {}")
                .action(ud),
        );
    app.run(args);
}

fn b64e(c: &Context) {
    println!("{}", base64::encode(&c.args[0]));
}

fn b64d(c: &Context) {
    let by = base64::decode(&c.args[0]).unwrap();
    let res = by.iter().map(|&s| s as char).collect::<String>();
    println!("{:?}",res);
}

fn ud(_c: &Context) {
    println!("{}", percent_decode(&_c.args[0].as_bytes()).decode_utf8().unwrap());
}

fn decode(input: &str) -> String {
    percent_decode(input.as_bytes()).decode_utf8().unwrap().to_string()
}

fn encode(input: &str) -> String {
    base64::encode(input)
}

#[cfg(test)]
mod tests {

    use crate::decode;
    use crate::encode;

    #[test]
    fn decode_space_ok() {
        let expected = "foo bar";
        let input = "foo%20bar";
        let actual = decode(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_ascii_ok() {
        let expected = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let input = r##"%20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let actual = decode(input);
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn decode_invalid_utf8_ng() {
        let input = "%93%FA%96%7B%8C%EA%0D%0A";
        decode(input);
    }

    #[test]
    fn base64_encode() {
        let input = "hello";
        encode(input);
    }
}

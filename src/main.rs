use std::env;
use seahorse::{App, Command, Context, Flag, FlagType};
use url::percent_encoding::percent_decode;
use url::{Url, Position};
use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli_tool [command] [x] [y]")
        .command(
            Command::new()
                .name("e")
                .usage("udrs e {}")
                .action(e),
        )
        .command(
            Command::new()
                .name("d")
                .usage("udrs d {}")
                .action(d),
        )  
        .command(ud_c()
        );
    app.run(args);
}

fn e(c: &Context) {
    if c.args.len() >= 1 {
        println!("{}", base64::encode(&c.args[0]));
    }
}

fn d(c: &Context) {
    if c.args.len() >= 1 {
        let by = base64::decode(&c.args[0]).unwrap();
        let res = by.iter().map(|&s| s as char).collect::<String>();
        println!("{:?}",res);
    }
}

fn ud_a(c: &Context) {
    if c.args.len() == 0 {
        std::process::exit(0);
    }
    let url = Url::parse(&c.args[0]).unwrap();
    if c.bool_flag("lpath") {
        let mut lp: String = url[Position::BeforePath..].to_string();
        lp.remove(0);
        println!("{}", lp);
    } else if c.bool_flag("domain") {
        println!("{}", url.domain().unwrap());
    } else if c.bool_flag("protocol") {
        println!("{}", url.scheme());
    } else if c.bool_flag("ip") {
        let hostname = url.domain().unwrap();
        let service = url.scheme();
        let hints = AddrInfoHints {
          socktype: SockType::Stream.into(),
          .. AddrInfoHints::default()
        };
        let sockets =
          getaddrinfo(Some(hostname), Some(service), Some(hints))
            .unwrap().collect::<std::io::Result<Vec<_>>>().unwrap();
        for socket in sockets {
          println!("{:?}", socket.sockaddr);
        }
    } else {
        println!("{}", percent_decode(c.args[0].as_bytes()).decode_utf8().unwrap());
    }
}

fn ud_c() -> Command {
    Command::new()
        .name("ud")
        .usage("cli ud [url...]")
        .action(ud_a)
        .flag(
            Flag::new(
                "lpath",
                "cli ud [url...] --lpath(-l)",
                FlagType::Bool,
                )
            .alias("l"),
            )
        .flag(
            Flag::new(
                "domain",
                "cli ud [url...] --domain(-d)",
                FlagType::Bool,
                )
            .alias("d"),
            )
        .flag(
            Flag::new(
                "protocol",
                "cli ud [url...] --protocol(-p)",
                FlagType::Bool,
                )
            .alias("p"),
            )
        .flag(
            Flag::new(
                "ip",
                "cli ud [url...] --ip(-i)",
                FlagType::Bool,
                )
            .alias("i"),
            )
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_space_ok() {
        let expected = "foo bar";
        let input = "foo%20bar";
        let actual = url::percent_encoding::percent_decode(input.as_bytes()).decode_utf8().unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    fn base64_encode() {
        let expected = "aGVsbG8gd29ybGQu";
        let actual = base64::encode("hello world.");
        assert_eq!(expected, actual);
    }
}

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
                .name("base64 encode")
                .usage("udrs b64e {}")
                .action(b64e),
        )
        .command(
            Command::new()
                .name("base64 decode")
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
    println!("{:?}", &base64::decode(&c.args[0]).unwrap()[..]);
}
fn ud(_c: &Context) {
    println!("{}", percent_decode(&_c.args[0].as_bytes()).decode_utf8().unwrap());
}

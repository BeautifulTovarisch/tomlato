use std::process;

use structopt::StructOpt;

mod lexer;

#[derive(StructOpt)]
struct Arg {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let Arg { path } = Arg::from_args();


}

use std::env;

#[derive(Debug, Default)]
pub struct CliArgs {
    pub selftest: bool,
    pub help: bool,
    pub version: bool,
    pub lang: Option<String>,
}

pub fn parse_args() -> CliArgs {
    let mut args = CliArgs::default();
    let mut iter = env::args().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--selftest" => args.selftest = true,
            "--help" | "-h" => args.help = true,
            "--version" | "-V" => args.version = true,
            "--lang" => {
                if let Some(val) = iter.next() {
                    args.lang = Some(val);
                }
            }
            _ => {}
        }
    }

    args
}

pub fn print_help() {
    println!(
        "Smart-Filee v{}
A local-first desktop file management suite

USAGE:
    smart_filee [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
    --selftest       Run automated self-tests and exit
    --lang <LANG>    Set UI language (en or th)
",
        env!("CARGO_PKG_VERSION")
    );
}

pub fn print_version() {
    println!("Smart-Filee v{}", env!("CARGO_PKG_VERSION"));
}

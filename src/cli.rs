use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Config file to loadg
    pub file: String,
}

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Option {
    /// enable fetching
    #[clap(short, long, value_parser, default_value_t = false)]
    pub fetch: bool,

    /// enable resolving
    #[clap(short, long, value_parser, default_value_t = false)]
    pub resolve: bool,

    /// enable generating
    #[clap(short, long, value_parser, default_value_t = false)]
    pub generate: bool,
}
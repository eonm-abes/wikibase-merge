use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author("Mathis EON <eon@abes.fr>"),
    version,
    about("Merge multiple wikibase items into a single one"),
    long_about = None
)]
pub struct Cli {
    #[clap(
        short,
        long,
        help = "A comma separated list of QItem IDs. All QItems will be merged with the last one. QItem IDS can also be piped from stdin.",
        required(atty::is(atty::Stream::Stdin))
    )]
    pub ids: Vec<String>,
    #[clap(
        short,
        long,
        help = "Set the wikibase API URL",
        env = "WIKIBASE_API_URL"
    )]
    pub url: String,
    #[clap(
        short,
        long,
        help = "Set the number of workers",
        env = "WIKIBASE_WORKERS"
    )]
    pub workers: Option<usize>,
}

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Nfpm(NfpmArgs),
}

#[derive(clap::Args, Debug)]
pub struct NfpmArgs {
    #[command(subcommand)]
    command: NfpmCommands,
}

#[derive(clap::Subcommand, Debug)]
pub enum NfpmCommands {
    Package(PackageArgs),
}

#[derive(clap::Args, Debug)]
pub struct PackageArgs {}

fn main() {
    let args = CliArgs::parse();
    dbg!(&args);
}

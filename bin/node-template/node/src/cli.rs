// --- crates ---
use structopt::StructOpt;
// --- substrate ---
use sc_cli::RunCmd;

/// An overarching CLI command definition.
#[derive(Debug, StructOpt)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub run: RunCmd,

	/// Load the boot configuration json file from <PATH>. Command line input will be overwritten by this.
	#[structopt(long = "conf", value_name = "PATH")]
	pub conf: Option<std::path::PathBuf>,
}

/// Possible subcommands of the main binary.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// A set of base subcommands handled by `sc_cli`.
	#[structopt(flatten)]
	Base(sc_cli::Subcommand),
}

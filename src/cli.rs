use clap::{Arg, Command};

pub fn setup() -> Command {
	Command::new("resin")
		.version("1.5.0")
		.author("Matt Gleich <email@mattglei.ch>")
		.about("Superfast CLI interface for the conventional commits commit format")
		.arg(
			Arg::new("all")
				.help("Run git add . before committing the the changes")
				.short('a')
				.long("all"),
		)
		.arg(
			Arg::new("push")
				.help("Run git push after committing the changes")
				.short('p')
				.long("push"),
		)
}

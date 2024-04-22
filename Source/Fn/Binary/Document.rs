#![allow(non_snake_case)]

use clap::{arg, Command};
use Library::Fn::Cache;

fn main() {
	let matches = Command::new("📄 Document.")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Nikola R. Hristov")
		.about("Build.")
		.get_matches();
}

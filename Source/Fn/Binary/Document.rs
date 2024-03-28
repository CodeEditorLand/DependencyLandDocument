#![allow(non_snake_case)]

use clap::{arg, Command};
use Library::Fn::Cache;

fn main() {
	let matches = Command::new("📄 Document Land.")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Nikola R. Hristov <nikola@playform.cloud>")
		.about("Build.")
		.get_matches();
}

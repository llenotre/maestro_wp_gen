//! This program allows to generate random wallpapers.
//! The program takes a primary and a secondary color, and optionaly a seed.
//!
//! The seed determines the wallpaper that will be generated. Running the program twice with the
//! same seed generates the same wallpaper.

#![feature(core_intrinsics)]

use std::env;
use std::process::exit;

mod util;

/// The path to the output file.
const OUTPUT: &str = "output.png";

/// Prints command line usage.
/// `bin` is the name of the binary in the command line.
fn print_usage(bin: &String) {
	eprintln!("usage: {} <primary> <secondary> [seed]", bin);
	eprintln!();
	eprintln!("`primary` and `secondary` must be hexadecimal colors");
	eprintln!("`seed` can be any string");
	eprintln!("The resulting wallpaper is written to `{}`", OUTPUT);
	exit(1);
}

/// Generates the wallpaper with the given colors `primary` and `secondary` and seed `seed`.
fn generate(_primary: &[u8; 3], _secondary: &[u8; 3], _seed: u64) {
	// TODO
	todo!();
}

/// Parses the given hexadecimal color `color`.
fn parse_color(_color: &String) -> Result<[u8; 3], ()> {
	// TODO
	todo!();
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 0 || args.len() > 4 {
		exit(1);
	}

	if args.len() < 3 {
		print_usage(&args[0]);
	} else {
		let primary = parse_color(&args[1]);
		if primary.is_err() {
			eprintln!("Invalid color primary `{}`!", args[1]);
			exit(1);
		}
		let primary = primary.unwrap();

		let secondary = parse_color(&args[2]);
		if secondary.is_err() {
			eprintln!("Invalid color secondary `{}`!", args[2]);
			exit(1);
		}
		let secondary = secondary.unwrap();

		let seed = if args.len() == 4 {
			// TODO Read seed
			todo!();
		} else {
			// TODO Generate a seed
			0
		};

		generate(&primary, &secondary, seed);
		// TODO Write to output file
	}
}

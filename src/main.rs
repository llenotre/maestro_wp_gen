//! This program allows to generate random wallpapers.
//! The program takes a primary and a secondary color, and optionaly a seed.
//!
//! The seed determines the wallpaper that will be generated. Running the program twice with the
//! same seed generates the same wallpaper.

#![feature(core_intrinsics)]

use std::env;
use std::process::exit;

use image::ImageBuffer;
use image::Rgb;

mod util;

/// The path to the output file.
const OUTPUT: &str = "output.jpg";

/// The interval between points on curves.
const CURVE_POINTS_INTERVAL: f32 = 100.;

/// Prints command line usage.
/// `bin` is the name of the binary in the command line.
fn print_usage(bin: &String) {
	eprintln!("usage: {} <width> <height> <primary> <secondary> [seed]", bin);
	eprintln!();
	eprintln!("`width` and `height` must be positive integers representing the size of the \
wallpaper");
	eprintln!("`primary` and `secondary` must be hexadecimal colors");
	eprintln!("`seed` can be any string");
	eprintln!("The resulting wallpaper is written to `{}`", OUTPUT);
	exit(1);
}

/// Structure representing a curve. A curve on this image is randomly generated and centered around
/// a linear function.
struct Curve {
	/// The linear function's y intercept.
	y_intercept: f32,
	/// The linear function's slope.
	slope: f32,

	/// points placed at a fixed interval along the linear function, with their displacements from
	/// the curve between -1. and 1.
	points: Vec<f32>,
}

/// Generates the wallpaper with the given colors `primary` and `secondary` and seed `seed`.
/// The wallpaper will have size `width`x`height`.
fn generate(width: u32, height: u32, primary: &[u8; 3], secondary: &[u8; 3], mut seed: u64)
	-> ImageBuffer<Rgb<u8>, Vec<u8>> {
	let mut curves = Vec::new();
	for i in 0..3 {
		// Generate random values. The only condition is that the line shows in the image
		let y_intercept = 0.; // TODO
		let slope = 0.; // TODO

		// Compute the number of points from the length of the curve inside the image
		let points_count = 0; // TODO
		let mut points = Vec::new();

		// Generating random point displacements
		for i in 0..points_count {
			points.push(0.); // TODO
		}

		curves.push(Curve {
			y_intercept,
			slope,

			points,
		});
	}

	ImageBuffer::from_fn(width, height, | x, y | {
		// TODO Replace with curves
		if 2 * x > y {
			image::Rgb(*primary)
		} else {
			image::Rgb(*secondary)
		}
	})
}

/// Parses the given hexadecimal color `hex`.
fn parse_color(mut hex: String) -> Result<[u8; 3], ()> {
	if hex.len() < 6 {
		return Err(());
	}

	if hex.chars().next().unwrap() == '#' {
		hex.remove(0);
	}

	if hex.len() != 6 {
		return Err(());
	}

	// Closure to convert from char value to hexadecimal value
	let convert = | v: u8 | {
		if v >= b'0' && v <= b'9' {
			v - b'0'
		} else if v >= b'A' && v <= b'F' {
			10 + (v - b'A')
		} else {
			10 + (v - b'a')
		}
	};

	let mut color: [u8; 3] = [0; 3];
	let mut iter = hex.chars();
	let mut i = 0;
	for c0 in iter.next() {
		let c1 = iter.next().unwrap();

		if !c0.is_ascii_hexdigit() || !c1.is_ascii_hexdigit() {
			return Err(());
		}

		color[i] = convert(c0 as u8) * 16 + convert(c1 as u8);
		i += 1;
	}

	Ok(color)
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 0 || args.len() > 6 {
		exit(1);
	}

	if args.len() < 5 {
		print_usage(&args[0]);
	} else {
		let width = args[1].parse::<u32>();
		if width.is_err() {
			eprintln!("Invalid width `{}`!", args[1]);
			exit(1);
		}
		let width = width.unwrap();

		let height = args[2].parse::<u32>();
		if height.is_err() {
			eprintln!("Invalid height `{}`!", args[2]);
			exit(1);
		}
		let height = height.unwrap();

		let primary = parse_color(args[3].clone());
		if primary.is_err() {
			eprintln!("Invalid color primary `{}`!", args[3]);
			exit(1);
		}
		let primary = primary.unwrap();

		let secondary = parse_color(args[4].clone());
		if secondary.is_err() {
			eprintln!("Invalid color secondary `{}`!", args[4]);
			exit(1);
		}
		let secondary = secondary.unwrap();

		let seed = if args.len() == 6 {
			// TODO Read seed
			todo!();
		} else {
			// TODO Generate a seed
			0
		};

		let wallpaper = generate(width, height, &primary, &secondary, seed);
		if wallpaper.save(OUTPUT).is_err() {
			eprintln!("Cannot write to file `{}`!", OUTPUT);
		}
	}
}

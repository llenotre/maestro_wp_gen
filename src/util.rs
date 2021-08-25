//! Utility functions.

use std::intrinsics::wrapping_add;
use std::intrinsics::wrapping_mul;

/// Linear congruential generator.
pub fn lcg(n: u64) -> u64 {
	wrapping_add(1442695040888963407, wrapping_mul(n, 6364136223846793005))
}

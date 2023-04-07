use std::error::Error;

use phf::*;

pub mod config;
pub mod caesar;
pub mod affine;
pub mod enigma;

use config::Config;
use config::scheme::Scheme;

/// The alphabet used by the encryption schemes (in upper case)
/// as an array of characters (for indexing).
pub const Σ: [char; 26] =
[
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
  'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

/// Returns the index of the given symbol in the alphabet (Σ).
///
/// The index of the first character in the alphabet is 0, the index of the
/// second character is 1, and so on, with the last one being 25. Thus, the
/// index of a character is the number of characters that come before it in the
/// alphabet.
pub const Γ: Map<char, usize> = phf_map!
{
  'A' => 0,  'B' => 1,
  'C' => 2,  'D' => 3,
  'E' => 4,  'F' => 5,
  'G' => 6,  'H' => 7,
  'I' => 8,  'J' => 9,
  'K' => 10, 'L' => 11,
  'M' => 12, 'N' => 13,
  'O' => 14, 'P' => 15,
  'Q' => 16, 'R' => 17,
  'S' => 18, 'T' => 19,
  'U' => 20, 'V' => 21,
  'W' => 22, 'X' => 23,
  'Y' => 24, 'Z' => 25,
};

/// Runs the encryption scheme specified in the given configuration.
///
/// # Arguments
/// * `config` - The configuration.
/// * `args` - The arguments.
pub fn run<T>(config: Config, args: T) -> Result<(), Box<dyn Error>>
where T: Iterator<Item = String>
{
  match config.scheme
  {
    Scheme::Caesar => caesar::run(config.mode, args),
    Scheme::Affine => affine::run(config.mode, args),
    Scheme::Enigma => enigma::run(config.mode, args)
  }
}

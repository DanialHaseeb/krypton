use std::error::Error;

pub mod config;
pub mod caesar;
pub mod affine;
pub mod enigma;

use config::Config;
use config::scheme::Scheme;

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

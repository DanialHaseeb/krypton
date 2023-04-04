pub mod scheme;
pub mod mode;

use scheme::Scheme;
use mode::Mode;

pub struct Config
{
  pub scheme: Scheme,
  pub mode: Mode
}

impl Config
{
  pub fn parse<T>(args: &mut T) -> Result<Config, &'static str>
  where T: Iterator<Item = String>
  {
    args.next();
    let mode = Mode::parse(args)?;
    let scheme = Scheme::parse(args)?;
    Ok(Config{ scheme, mode })
  }
}

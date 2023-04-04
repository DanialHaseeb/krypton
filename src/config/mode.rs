#[derive(Debug, PartialEq)]
pub enum Mode
{
  Encrypt,
  Decrypt,
  Analyse
}

impl Mode
{
  pub fn parse<T>(args: &mut T) -> Result<Mode, &'static str>
  where T: Iterator<Item = String>
  {
    if let Some(scheme) = args.next()
    {
      use Mode::*;
      let scheme = scheme.to_lowercase();
      let scheme = scheme.as_str();
      match scheme
      {
        "encrypt" => Ok(Encrypt),
        "decrypt" => Ok(Decrypt),
        "analyse" => Ok(Analyse),
        _ => Err("Unknown mode of operation. 🤔")
      }
    }
    else
    { Err("No mode of operation provided. 🧐") }
  }
}

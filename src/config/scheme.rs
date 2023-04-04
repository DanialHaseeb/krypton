#[derive(Debug)]
pub enum Scheme
{
  Caesar,
  Affine,
  Enigma
}

impl Scheme
{
  pub fn parse<T>(args: &mut T) -> Result<Scheme, &'static str>
  where T: Iterator<Item = String>
  {
    if let Some(scheme) = args.next()
    {
      use Scheme::*;
      let scheme = scheme.to_lowercase();
      let scheme = scheme.as_str();
      match scheme
      {
        "caesar" => Ok(Caesar),
        "affine" => Ok(Affine),
        "enigma" => Ok(Enigma),
        _ => Err("scheme => Unknown scheme. 🤔")
      }
    }
    else
    { Err("scheme => No encryption scheme provided. 🧐") }
  }
}

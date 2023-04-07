use crate::Γ;

/// The type of a rotor.
#[derive(Debug)]
pub enum Reflector
{
  A,
  B,
  C
}

impl Reflector
{
  pub fn parse<T>(args: &mut T) -> Result<Reflector, &'static str>
  where T: Iterator<Item = String>
  {
    if let Some(kind) = args.next()
    {
      use Reflector::*;

      let kind = kind.to_uppercase();
      match kind.as_str()
      {
        "A" => Ok(A),
        "B" => Ok(B),
        "C" => Ok(C),
        _ => Err("Invalid reflector type. 🔁")
      }
    }
    else
    { Err("Missing reflector type. 🧐") }
  }

  pub fn wiring(&self) -> [usize; 26]
  {
    use Reflector::*;

    let mapping = match self
    {
      A => "EJMZALYXVBWFCRQUONTSPIKHGD",
      B => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
      C => "FVPJIAOYEDRZXWGCTKUQSBNMHL"
    };

    let mut wiring = [0; 26];
    for (γ, σ) in mapping.chars().enumerate()
    { wiring[γ] = Γ[&σ]; }

    wiring
  }
}

use std::fmt;

use crate::Γ;

/// The type of a rotor.
#[derive(Debug, Clone, Copy)]
pub enum Kind
{
  A,
  B,
  C
}

impl Kind
{
  pub fn parse<T>(args: &mut T) -> Result<Kind, &'static str>
  where T: Iterator<Item = String>
  {
    if let Some(kind) = args.next()
    {
      use Kind::*;

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
    use Kind::*;

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

#[derive(Clone)]
pub struct Reflector
{
  pub kind: Kind,
  pub wiring: [usize; 26]
}

impl fmt::Debug for Reflector
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  { write!(f, "{:?}", self.kind) }
}

impl Reflector
{
  pub fn parse<T>(args: &mut T) -> Result<Reflector, &'static str>
  where T: Iterator<Item = String>
  {
    let kind = Kind::parse(args)?;
    let wiring = kind.wiring();

    Ok(Reflector{ kind, wiring })
  }

  pub fn new(kind: Kind) -> Reflector
  {
    let wiring = kind.wiring();
    Reflector{ kind, wiring }
  }

  pub fn default() -> Reflector
  { Self::new(Kind::A) }
}

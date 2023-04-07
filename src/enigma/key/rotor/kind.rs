use crate::Γ;

/// The type of a rotor.
#[derive(Debug)]
pub enum Kind
{
  I,
  II,
  III,
  IV,
  V
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
        "I"   => Ok(I),
        "II"  => Ok(II),
        "III" => Ok(III),
        "IV"  => Ok(IV),
        "V"   => Ok(V),
        _ => Err("Invalid rotor type. ⚙️")
      }
    }
    else
    { Err("Missing rotor type(s). 🧐") }
  }

  pub fn notch(&self) -> usize
  {
    use Kind::*;

    match self
    {
      I   => Γ[&'Q'],
      II  => Γ[&'E'],
      III => Γ[&'V'],
      IV  => Γ[&'J'],
      V   => Γ[&'Z'],
    }
  }

  pub fn wiring(&self) -> [usize; 26]
  {
    use Kind::*;

    let wiring =
    match self
    {
      I   => "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
      II  => "AJDKSIRUXBLHWTMCQGZNPYFVOE",
      III => "BDFHJLCPRTXVZNYEIWGAKMUSQO",
      IV  => "ESOVPZJAYQUIRHXLNFTGKDCMWB",
      V   => "VZBRGITYUPSDNHLXAWMJQOFECK",
    };

    let mut output = [0; 26];
    for (i, σ) in wiring.chars().enumerate()
    { output[i] = Γ[&σ]; }

    output
  }

  pub fn inverse_wiring(&self) -> [usize; 26]
  {
    let mut inverse_wiring = [0; 26];
    for (u, &v) in self.wiring().iter().enumerate()
    { inverse_wiring[v] = u; }
    inverse_wiring
  }
}

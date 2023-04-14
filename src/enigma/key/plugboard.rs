use std::fmt;

use crate::Σ;
use crate::Γ;

#[derive(Clone, Copy)]
pub struct Plugboard
{ pub wiring: [usize; 26] }

impl fmt::Debug for Plugboard
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  {
    let mut output = String::new();

    for (a, b) in self.wiring.iter().enumerate()
    {
      if a >= *b
      { continue; }

      output.push(Σ[a]);
      output.push('⇆');
      output.push(Σ[*b]);
      output.push(' ');
    }

    write!(f, "{output}")
  }
}

impl Plugboard
{
  pub fn connect(&mut self, a: usize, b: usize)
  {
    self.wiring[a] = b;
    self.wiring[b] = a;
  }

  pub fn disconnect(&mut self, a: usize, b: usize)
  {
    assert!(self.wiring[a] == b);
    assert!(self.wiring[b] == a);

    self.wiring[a] = a;
    self.wiring[b] = b;
  }

  pub fn has_plugged(&self, a: usize) -> bool
  { self.wiring[a] != a }

  pub fn parse<T>(args: &mut T) -> Result<Plugboard, &'static str>
  where T: Iterator<Item = String>
  {
    let mut plugboard = Plugboard::default();
    for _ in 0..13
    {
      if let Some(arg) = args.next()
      {
        if arg.as_str() == "--"
        { break; }

        let mut chars = arg.chars();
        if let (Some(a), Some(b)) = (chars.next(), chars.next())
        {
          if let Some(_) = chars.next()
          { return Err("Plugboard connections must be two letters. ❌"); }

          if !(a.is_ascii_alphabetic() && b.is_ascii_alphabetic())
          { return Err("Plugboard connections must be two letters. ❌"); }

          let a = Γ[&a];
          let b = Γ[&b];

          if plugboard.has_plugged(a) || plugboard.has_plugged(b)
          { return Err("Cannot plug twice into one socket. 🎛️"); }

          plugboard.connect(a, b);
        }
        else
        { return Err("Plugboard connections must be two letters. ❌"); }
      }
    }

    Ok(plugboard)
  }

  pub fn default() -> Plugboard
  {
    let mut wiring = [0; 26];
    for i in 0..26
    { wiring[i] = i; }
    Plugboard{ wiring }
  }
}

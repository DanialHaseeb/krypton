use crate::Γ;

#[derive(Debug)]
pub struct Plugboard
{ pub wiring: [usize; 26] }

impl Plugboard
{
  pub fn parse<T>(args: &mut T) -> Result<Plugboard, &'static str>
  where T: Iterator<Item = String>
  {
    let mut wiring = [0; 26];
    for i in 0..26
    { wiring[i] = i; }
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

          if !(a.is_ascii_alphabetic() && (b.is_ascii_alphabetic()))
          { return Err("Plugboard connections must be two letters. ❌"); }

          let a = Γ[&a];
          let b = Γ[&b];

          if (wiring[a] != a) || (wiring[b] != b)
          { return Err("Cannot plug twice into one socket. 🎛️"); }

          wiring[a] = b;
          wiring[b] = a;
        }
        else
        { return Err("Plugboard connections must be two letters. ❌"); }
      }
    }

    Ok(Plugboard{ wiring })
  }
}

use std::error::Error;

pub mod rotor;
pub mod reflector;
pub mod plugboard;

use rotor::Rotor;
use reflector::Reflector;
use plugboard::Plugboard;

#[derive(Debug, Clone)]
pub struct Key
{
  pub rotors: [Rotor; 3],
  pub reflector: Reflector,
  pub plugboard: Plugboard
}

impl Key
{
  pub fn parse<T>(mut args: T) -> Result<Key, Box<dyn Error>>
  where T: Iterator<Item = String>
  {
    let rotors =
    {
      let left   = Rotor::parse(&mut args)?;
      let middle = Rotor::parse(&mut args)?;
      let right  = Rotor::parse(&mut args)?;
      [left, middle, right]
    };

    let reflector = Reflector::parse(&mut args)?;

    let plugboard = Plugboard::parse(&mut args)?;

    Ok(Key{ rotors, reflector, plugboard })
  }

  pub fn default() -> Key
  {
    Key
    {
      rotors:
      [
        Rotor::new(rotor::kind::Kind::I, 0, 0),
        Rotor::new(rotor::kind::Kind::II, 0, 0),
        Rotor::new(rotor::kind::Kind::III, 0, 0)
      ],
      reflector: Reflector::new(reflector::Kind::A),
      plugboard: Plugboard::default()
    }
  }

  pub fn rotate_rotors(&mut self)
  {
    let mut step_next = true;
    for rotor in self.rotors.iter_mut().rev()
    {
      if step_next
      { step_next = rotor.rotate(); }
      else
      { break; }
    }
  }
}

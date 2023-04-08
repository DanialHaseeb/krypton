use std::error::Error;

pub mod rotor;
pub mod reflector;
pub mod plugboard;

use rotor::Rotor;
use rotor::kind::Kind;
use plugboard::Plugboard;
use reflector::Reflector;

#[derive(Debug)]
pub struct Key
{
  pub rotors: [Rotor; 3],
  pub reflector: [usize; 26],
  pub plugboard: [usize; 26]
}

impl Key
{
  pub fn parse<T>(mut args: T) -> Result<Key, Box<dyn Error>>
  where T: Iterator<Item = String>
  {
    let rotors =
    {
      let rotor1 = Rotor::parse(&mut args)?;
      let rotor2 = Rotor::parse(&mut args)?;
      let rotor3 = Rotor::parse(&mut args)?;
      [rotor1, rotor2, rotor3]
    };

    let reflector = Reflector::parse(&mut args)?;
    let reflector = reflector.wiring();

    let plugboard = Plugboard::parse(&mut args)?;
    let plugboard = plugboard.wiring;

    Ok(Key{ rotors, reflector, plugboard })
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

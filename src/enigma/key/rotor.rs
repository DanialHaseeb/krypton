use std::error::Error;
use std::fmt;

pub mod kind;
pub mod position;
pub mod ring_setting;

use crate::Σ;
use kind::Kind;

#[derive(Clone, Copy)]
pub struct Rotor
{
  /// The rotor's type
  pub kind: Kind,
  /// The rotor's wiring
  pub wiring: [usize; 26],
  /// The rotor's inverse wiring
  pub inverse_wiring: [usize; 26],
  /// The rotor's turnover position
  pub notch: usize,
  /// The rotor's current position
  pub position: usize,
  /// The rotor's ring setting
  pub ring_setting: usize
}

impl fmt::Debug for Rotor
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  {
    let kind = self.kind;
    let position = Σ[self.position];
    let ring_setting = Σ[self.ring_setting];

    write!(f, "{:?} {:?} {:?}", kind, position, ring_setting)
  }
}

impl Rotor
{
  pub fn new(kind: Kind, position: usize, ring_setting: usize) -> Rotor
  {
    let wiring = kind.wiring();
    let inverse_wiring = kind.inverse_wiring();
    let notch = kind.notch();

    Rotor{ kind, wiring, inverse_wiring, notch, position, ring_setting }
  }

  pub fn parse<T>(args: &mut T) -> Result<Rotor, Box<dyn Error>>
  where T: Iterator<Item = String>
  {
    let kind = Kind::parse(args)?;
    let wiring = kind.wiring();
    let inverse_wiring = kind.inverse_wiring();
    let notch = kind.notch();
    let position = position::parse(args)?;
    let ring_setting = ring_setting::parse(args)?;

    Ok(Rotor{ kind, wiring, inverse_wiring, notch, position, ring_setting })
  }

  pub fn rotate(&mut self) -> bool
  {
    self.position = (self.position + 1) % 26;
    self.position == self.notch
  }

  pub fn map(&self, γ: usize) -> usize
  {
    let shift = 26 + self.position - self.ring_setting;
    let γ = (γ + shift) % 26;
    let γ = self.wiring[γ];
    let reverse_shift = 26 + self.ring_setting - self.position;
    (γ + reverse_shift) % 26
  }

  pub fn inverse_map(&self, γ: usize) -> usize
  {
    let shift = 26 + self.position - self.ring_setting;
    let γ = (γ + shift) % 26;
    let γ = self.inverse_wiring[γ];
    let reverse_shift = 26 + self.ring_setting - self.position;
    (γ + reverse_shift) % 26
  }
}

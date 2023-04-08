use std::error::Error;

pub mod kind;
pub mod position;
pub mod ring_setting;

use kind::Kind;

#[derive(Debug)]
pub struct Rotor
{
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

impl Rotor
{
  pub fn new(kind: Kind, position: usize, ring_setting: usize) -> Rotor
  {
    let wiring = kind.wiring();
    let inverse_wiring = kind.inverse_wiring();
    let notch = kind.notch();

    Rotor{ wiring, inverse_wiring, notch, position, ring_setting }
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

    Ok(Rotor{ wiring, inverse_wiring, notch, position, ring_setting })
  }

  pub fn defaults() -> [Rotor; 5]
  {
    let i = Rotor::new(Kind::I, 0, 0);
    let ii = Rotor::new(Kind::II, 0, 0);
    let iii = Rotor::new(Kind::III, 0, 0);
    let iv = Rotor::new(Kind::IV, 0, 0);
    let v = Rotor::new(Kind::V, 0, 0);

    [i, ii, iii, iv, v]
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

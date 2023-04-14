use crate::Γ;

pub mod trigrams;

pub fn score(plaintext: &String) -> f64
{
  let mut histogram = [0; 26];
  for σ in plaintext.chars()
  { histogram[Γ[&σ]] += 1; }

  let n = plaintext.len();
  let mut total = 0.0;
  for &count in histogram.iter()
  { total += (count * (count - 1)) as f64; }

  total / (n * (n - 1)) as f64
}
  // let mut total = 0.0;

  // let mut symbols = plaintext.chars();
  // if let Some(first) = symbols.next()
  // {
  //   let mut first = Γ[&first] as u8;
  //   if let Some(second) = symbols.next()
  //   {
  //     let mut second = Γ[&second] as u8;
  //     while let Some(third) = symbols.next()
  //     {
  //       let third = Γ[&third] as u8;
  //       let key = [first, second, third];
  //       let score = TRIGRAMS.get(&key).unwrap_or(&-8.0);
  //       total += score;

  //       first = second;
  //       second = third;
  //     }
  //     total
  //   }
  //   else
  //   { f64::NEG_INFINITY }
  // }
  // else
  // { f64::NEG_INFINITY }

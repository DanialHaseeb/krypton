use std::error::Error;

pub fn parse<T>(args: &mut T) -> Result<usize, Box<dyn Error>>
where T: Iterator<Item = String>
{
  if let Some(ring_setting) = args.next()
  {
    let ring_setting: isize = ring_setting.parse()?;
    let ring_setting = (ring_setting % 26) as usize;
    Ok(ring_setting)
  }
  else
  { Err("Missing ring setting(s). 🧐")? }
}

use std::env;
use std::process;

use krypton::config::Config;

fn main()
{
  let mut args = env::args();
  let config = Config::parse(&mut args)
    .unwrap_or_else
    (|message|
    {
      eprintln!("Parse Error: {message}");
      process::exit(1);
    });

  if let Err(message) = krypton::run(config, args)
  {
    eprintln!("Error: {message}");
    process::exit(1);
  }
}

use std::io;
use crate::authenticator::login::LogIn;

mod authenticator;

fn main() -> io::Result<()>  {
   let obj = LogIn::new("Hello","World")?;
   Ok(())
}

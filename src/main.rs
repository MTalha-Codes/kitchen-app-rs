use std::io;
use crate::authenticator::login::LogIn;

mod authenticator;

fn main() -> io::Result<()>  {
   let obj = LogIn::new("Hello","World")?;
   match obj.authenticate_user() {
         Ok(_) => println!("Done"),
         Err(_) => println!("failed")  
   };
   Ok(())
}

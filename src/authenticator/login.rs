use std::fs::File;
use std::{env, fs, io};

pub struct LogIn
{ 
    user_name: String,
    password: String,
    file_path: String
}

impl LogIn {
    
    /// 
    pub fn new(_user_name:&str, _password:&str) -> io::Result<Self> {
        let construct_and_return_path_for_user_credentials_file = ||
            {
                let mut cwd = env::current_dir().expect("Failed to fetch to current working directory !");
                cwd.push("Managed");
                cwd.push("UserData");
                cwd.set_file_name(_user_name);
                cwd.set_extension("cred");
                cwd.to_path_buf().to_str().unwrap().to_string()
            };
        
        Ok(LogIn{
            user_name: _user_name.to_string(),
            password: _password.to_string(),
            file_path: construct_and_return_path_for_user_credentials_file() 
        })
    }
    
    fn check_user_existence(&self) ->bool
    {
       match File::open(&self.file_path) {
          Ok(_) =>  true,
          Err(_) => false 
       }
    }
    
    pub fn authenticate_user(&self) {
        todo!("to be implemented")
    }
}
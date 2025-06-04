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
                cwd.push(_user_name);
                cwd.set_file_name(_user_name);
                cwd.set_extension("cred");
                cwd.to_str().unwrap().to_string()
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
    
    pub fn authenticate_user(&self) -> Result<&str,&str> {
        println!("self.file_path = {}",self.file_path);
        println!("self.user_name = {}",self.user_name);
        println!("self.password = {}",self.password);
        if !self.check_user_existence() {
            Err("Failed to authenticate user (why? user doesn't exists)")
        } else {
            let data = fs::read_to_string(&self.file_path).expect("an unexpected error occurred while reading a file");
            let pipe_index = data.find("|").expect("user file is corrupted");
            let user_name = &data[0..pipe_index];
            let password = &data[pipe_index+1..data.len()];
            let final_enum = if user_name==self.user_name && password == self.password {
                Ok("Login Successful")
            }
            else { 
                Err("Incorrect Credentials Provided")
            };
            final_enum
        }
    }
}
use std::fs::File;
use std::{env, fs};

///# Struct: LogIn
///* Type: struct
///* Visibility: public
///## Members or Fields
///1. **user_name**: this member keeps the user_name typed by the user.
///2. **password**: this member keeps the password typed by the user.  
///3. **file_path**: this member notes the path that where the file is stored that contains necessary data for login.
///### Examples:
///  In ***main.rs*** the struct is used in this way.
/// ```rs
/// let login_object = authenticator::login::Login::new("UserName","Password");
/// match login_object.authenticate_user() 
/// {
///     Ok(_) => println!("Login Successful"),
///     Err(_) => println!("Login Failed")
/// }
/// ```
pub struct LogIn
{ 
    user_name: String,
    password: String,
    file_path: String
}

impl LogIn {
    
    /// # The `new` method.
    /// this function (in the struct `LogIn`) is used to initialize the fields; you can say this is a constructor.
    /// ## Parameters:
    /// 1. **_user_name**
    /// 2. **_password** 
    /// ### Parameters Details:
    /// Both `_user_name` and `_password` are of type `&str`; or if you are from C++, then you'd consider it like `const char*`.
    /// ## Returns
    /// The `new` function returns a `Self` instance which contains the initialized struct object.
    pub fn new(_user_name:&str, _password:&str) -> Self {
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
        
        LogIn{
            user_name: _user_name.to_string(),
            password: _password.to_string(),
            file_path: construct_and_return_path_for_user_credentials_file() 
        }
    }
    /// # The `check_user_existence` method.
    /// As clear from its name; this function verifies the existence of user by checking the user credentials file.
    /// ## Returns
    /// This method returns a bool
    /// * `false` if the user credential file doesn't exist.
    /// * `true` if the user credential file exist.
    fn check_user_existence(&self) ->bool
    {
       match File::open(&self.file_path) {
          Ok(_) =>  true,
          Err(_) => false 
       }
    }
    /// # The `authenticate_user` method.
    /// This function tries to authenticate a user into the app based on given credentials.
    /// ## Returns
    /// This function returns a `Result` enum for correct error handling for cases like. 
    /// * The user credential file doesn't exist which directly implies that the user doesn't exist in the platform.
    /// * The program hasn't permission to read the user credential file.
    /// * The provided credentials are not correct. 
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
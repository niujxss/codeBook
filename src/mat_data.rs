

use serde::{Serialize, Deserialize};
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct MatData{
    username : String,
    password : String,
    notes:String,
}


impl MatData{
    pub fn new(username : String, password : String, notes : String) -> MatData{
        MatData{
            username,
            password,
            notes,
        }

    }

    pub fn showdata(&self) {
        println!("\n");
        println!("\t 用户名：{}",self.username);
        println!("\t 密码：{}",self.password);
        println!("\t 备注：{}", self.notes);

    }
}
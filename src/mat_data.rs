

use serde::{Serialize, Deserialize};
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct MatData{
    username : String,
    password : String,
    notes:String,
    pub id:usize,
}


impl MatData{
    pub fn new(username : String, password : String, notes : String,id:usize) -> MatData{
        MatData{
            username,
            password,
            notes,
            id,
        }

    }

    pub fn showdata(&self) {
        println!("\n编号为{}的密码信息",self.id);
        println!("\t 用户名：{}",self.username);
        println!("\t 密码：{}",self.password);
        println!("\t 备注：{}", self.notes);

    }

    pub fn comp_id(&self,id:usize) -> Option<Self>{
        if self.id == id {
             Some(self.clone())
        } else {
             None
        }
    }

    pub fn comp_name(&self,name:&str) -> Option<Self>{
        if self.username.to_lowercase().contains(name.to_lowercase().as_str()) {
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn comp_notes(&self,notes:&str) -> Option<Self>{
        if self.notes.to_lowercase().contains(notes.to_lowercase().as_str()) {
            Some(self.clone())
        } else {
            None
        }
    }
    pub fn update_name(&mut self,name:String) {
        self.username = name;
    }

    pub fn update_passwd(&mut self,passwd:String) {
        self.password = passwd;
    }

    pub fn update_notes(&mut self,notes:String) {
        self.notes = notes;
    }
}
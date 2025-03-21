mod code_book;
mod mat_data;
use std::io;

use crate::code_book::CodeBook;
use std::env;

fn main() {
    println!("Hello, 欢迎使用本工具");

    let mut codeboot = code_book::CodeBook::load_or_new();

    loop {
        let input = menuconfig();
        match input {
            1 => {
                insert_data(&mut codeboot);
            },
            2 => {
                show_code_book(&codeboot);
            },
            3 => {

                add_file(&mut codeboot);
            },
            _ => {
                println!("ByeBye!!");
                break;
            },
        }
    }

}

fn add_file(codebook:&mut CodeBook) {
    let mut filepath = String::new();
    let current_dir = env::current_dir().expect("无法获取当前目录");
    println!("当前工作目录: {:?}", current_dir);
    println!("请输入相对路径下的文件名（后缀名为.ck）：");

    io::stdin().read_line(&mut filepath).expect("读取输入的数据错误");

    let filename = filepath.trim().to_string();

    if filename.ends_with(".ck") {

        let filepath = current_dir.join(&filename);
        codebook.add_from_file(filepath);

    } else {
        println!("文件名格式不正确")
    }

}

fn insert_data(code_book: &mut CodeBook) {
    let mut name = String::new();
    let mut password = String::new();

    println!("请输入用户名：");
    io::stdin().read_line(&mut name).expect("读取输入的数据错误");

    println!("请输入密码：");
    io::stdin().read_line(&mut password).expect("读取输入的数据错误");

    let mut notes = String::new();
    println!("请输入备注信息");
    io::stdin().read_line(&mut notes).expect("读取输入的数据错误");

    code_book.add(name, password, notes);

}

fn show_code_book(code_book: &CodeBook) {
    code_book.showdata();
}

fn menuconfig() -> u8 {
    println!("============== 私人定制密码本 ===============");
    println!("= 请输入功能编号                             ");
    println!("= 1、创建一条密码                               ");
    println!("= 2、查看所有密码");
    println!("= 3、导入密码本");
    println!("= 输入其他数据退出");
    let mut inputstr = String::new();
    io::stdin().read_line(&mut inputstr).expect("Failed to read line");

    let input:u8 = match inputstr.trim().parse() {
        Ok(num) => {
            println!("输入的参数为{}",num);
            num
        },
        Err(_) => {
            println!("退出");
            0
        },
    };
    return input;
}
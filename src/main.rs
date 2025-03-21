mod code_book;
mod mat_data;

use crate::code_book::CodeBook;
use crate::code_book::OptionType;
use std::env;
use std::io::{self, Write};
use clearscreen::clear;


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
            4 => {

                find_book(&codeboot);
            },
            5 => {
                change_boot(&mut codeboot);
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
    println!("= 4、查找密码");
    println!("= 5、修改密码本");
    println!("= 输入其他数据退出");
    let mut inputstr = String::new();
    io::stdin().read_line(&mut inputstr).expect("Failed to read line");

    let input:u8 = match inputstr.trim().parse() {
        Ok(num) => {
            num
        },
        Err(_) => {
            println!("退出");
            0
        },
    };
    return input;
}

fn find_book(code_book: &CodeBook) {
    clear().expect("无法清空屏幕");
    println!("请选择查找的方式：");
    println!("1. 通过ID查找");
    println!("2. 通过名字查找");
    println!("3. 通过note查找");
    let mut inputstr = String::new();
    io::stdin().read_line(&mut inputstr).expect("Failed to read line");
    let input:u8 = match inputstr.trim().parse() {
        Ok(num) => {
            num
        },
        Err(_) => {return}
    };

    match input {
        1 => {
            println!("请输入ID：");
            let mut inputstr = String::new();
            io::stdin().read_line(&mut inputstr).expect("Failed to read line");
            let input:usize = match inputstr.trim().parse() {
                Ok(num) => {
                    num
                },
                Err(_) => {println!("输入的编号异常");return}
            };
            code_book.find(OptionType::ID(input)).unwrap();
        },
        2 => {
            println!("请输入要查询的数据：");
            let mut inputstr = String::new();
            io::stdin().read_line(&mut inputstr).expect("Failed to read line");
            let input = inputstr.trim();
            code_book.find(OptionType::NAME(input)).unwrap();
        },
        3 => {
            println!("请输入要查询的数据：");
            let mut inputstr = String::new();
            io::stdin().read_line(&mut inputstr).expect("Failed to read line");
            let input = inputstr.trim();
            code_book.find(OptionType::NOTES(input)).unwrap();
        },
        _ => {
            println!("输入错误!!");
        }
    }

}

fn change_boot(code_book: &mut CodeBook) {
    clear().expect("无法清空屏幕");
    println!("请选择操作方式：");
    println!("1. 根据ID删除密码");
    println!("2. 根据ID修改密码参数");
    let mut inputstr = String::new();
    io::stdin().read_line(&mut inputstr).expect("Failed to read line");
    let input:u8 = match inputstr.trim().parse() {
        Ok(num) => {
            num
        },
        Err(_) => {return}
    };
    match input {
        1 => {
            change_boot_rm(code_book);
        },
        2 => {
            change_boot_changedata(code_book);
            },
        _ => {
            println!("输入错误");
        }
    }
}

fn change_boot_rm(code_book: & mut CodeBook){
    println!("请输入ID：");
    let mut inputstr = String::new();
    io::stdin().read_line(&mut inputstr).expect("Failed to read line");
    let input:usize = match inputstr.trim().parse() {
        Ok(num) => {
            num
        },
        Err(_) => {return}
    };

    let ret = code_book.remove_by_id(input);

    match ret {
        Ok(_) => {println!("删除成功")},
        Err(_) => {println!("删除失败");}
    }
}

fn change_boot_changedata(code_book: & mut CodeBook){
    println!("请输入ID：");
    let mut inputstr = String::new();
    io::stdin().read_line(&mut inputstr).expect("Failed to read line");
    let inputid:usize = match inputstr.trim().parse() {
        Ok(num) => {
            num
        },
        Err(_) => {return}
    };

    let ret = code_book.find(OptionType::ID(inputid));

    match ret {
        Ok(_) => {

            let name : Option<String> ;
            let passwd : Option<String> ;
            let note : Option<String> ;

            let ret = get_yes_no("是否修改用户名？").unwrap();
            if ret {

                let input_name = get_input_message("请输入新用户名").unwrap();
                name = Some(input_name);
            } else {
                name = None;
            }

            let ret = get_yes_no("是否修改密码？").unwrap();
            if ret {
                let input_passwd = get_input_message("请输入新密码").unwrap();
                passwd = Some(input_passwd);
            } else {
                passwd = None;
            }

            let ret = get_yes_no("是否修改备注？").unwrap();
            if ret {
                let input_note = get_input_message("请输入新备注").unwrap();
                note = Some(input_note);
            } else {
                note = None;
            }

            let ret = code_book.update_by_id(inputid,name,passwd,note);

            match ret {
                Ok(_) => {println!("修改成功！！")},
                Err(_) => {println!("修改失败!!!")},
            }

        },
        Err(_) => {println!("ID异常，无法进行修改")}
    }


}

fn get_yes_no(message:&str) ->Result<bool,()> {
    let mut input = String::new();
    loop {
        print!("{}[y/n]: ", message);
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let normalized = input.trim().to_lowercase();
        match normalized.as_str() {
            ""|"y" | "yes" => { return Ok(true)},
            "n" | "no" => { return Ok(false)},
            _ => {println!("无效输入，请重新输入y/n");continue;}
        }
    }
}

fn get_input_message(message:&str) ->Result<String,()> {
    let mut input = String::new();

        print!("{}: ", message);
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let normalized = input.trim().to_lowercase();
        Ok(normalized)
}


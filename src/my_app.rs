
use eframe::egui;
use crate::code_book::CodeBook;
use crate::code_book::OptionType;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use log::{Log, LevelFilter, info};
use std::sync::Arc;

// --- 全局日志缓冲区 ---
lazy_static! {
    static ref LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

struct GuiLogger;
impl Log for GuiLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool { true }

    fn log(&self, record: &log::Record) {
        let log = format!("{}", record.args());
        LOGS.lock().push(log);
    }

    fn flush(&self) {}
}

#[derive(PartialEq,Debug)]
enum Page {
    Main,
    SubPage1,
    SubPage2,
    SubPage3,
    SubPage4,
    SubPage4_1,
    SubPage4_2,
    SubPage4_3,
    SubPage5,
    SubPage5_1,
    SubPage5_2,
    MessagePage,
}

pub struct MyApp {
    current_page: Page,  // 当前显示的页面
    codebook: CodeBook,
    show_logs: bool,
    sub_page1data: SubPage1Data,
    sub_page4_1data: SubPage4_1Data,
    sub_page4_2data: SubPage4_2Data,
    sub_page4_3data: SubPage4_3Data,
    sub_page5_1data: SubPage4_1Data,
    sub_page5_2data: SubPage5_2Data,
}

struct SubPage1Data {
    input_name :String,
    input_passwd:String,
    input_notes:String,
}

struct SubPage4_1Data {
    input_id:usize,
    input_id_str:String,
}
struct SubPage4_2Data {
    input_name:String,
}

struct SubPage4_3Data {
    input_notes:String,
}

struct SubPage5_2Data {
    inputid : SubPage4_1Data,
    input_name :String,
    input_passwd:String,
    input_notes:String,
    input_name_status:bool,
    input_passwd_status:bool,
    input_notes_status:bool,

}

impl SubPage5_2Data {
    fn new() -> Self {
        Self{
            inputid:SubPage4_1Data::new(),
            input_name:String::new(),
            input_passwd: String::new(),
            input_notes: String::new(),
            input_name_status:false,
            input_passwd_status:false,
            input_notes_status:false,
        }
    }
    fn clear(&mut self) {
        self.inputid.input_id = 0;
        self.inputid.input_id_str.clear();
        self.input_name.clear();
        self.input_passwd.clear();
        self.input_notes.clear();
        self.input_name_status = false;
        self.input_passwd_status = false;
        self.input_notes_status = false;
    }
}
impl SubPage4_1Data {
    fn new()->Self{
        Self {
            input_id:0,
            input_id_str:String::new(),
        }
    }
}
impl SubPage4_2Data{
    fn new()->Self{
        Self{
            input_name:String::new(),
        }
    }
}

impl SubPage4_3Data {
    fn new()->Self{
        Self {
            input_notes:String::new(),
        }
    }
}

impl SubPage1Data {
    fn new() -> Self {
        Self{
            input_name:String::new(),
            input_passwd:String::new(),
            input_notes:String::new(),
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        // 初始化日志系统
        log::set_boxed_logger(Box::new(GuiLogger)).unwrap();
        log::set_max_level(LevelFilter::Info);

        Self {
            current_page: Page::Main,
            codebook: CodeBook::load_or_new(),
            show_logs: true,
            sub_page1data: SubPage1Data::new(),
            sub_page4_1data: SubPage4_1Data::new(),
            sub_page4_2data: SubPage4_2Data::new(),
            sub_page4_3data: SubPage4_3Data::new(),
            sub_page5_1data: SubPage4_1Data::new(),
            sub_page5_2data: SubPage5_2Data::new(),
        }
    }
}

impl eframe::App for MyApp { //为 MyApp 实现 eframe::App trait,这样就能将自己的APP接入eframe框架
    //通过实现这个 trait，框架可以自动处理窗口事件循环、渲染调度等底层逻辑

    //唯一必须实现的函数update, 作用：每帧调用一次，负责绘制界面和处理用户输入。
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //参数1：应用状态，可修改
        //参数2：EGUI 上下文，提供输入事件、绘图接口等
        //参数3：原生窗口控制（如修改窗口标题、关闭窗口）

        match self.current_page {
            Page::Main => {
                egui::CentralPanel::default().show(ctx,|ui|{
                    ui.heading("主界面");
                    ui.separator();

                    // 添加了四个选项按钮，并描述了按钮被按下后的动作
                    if ui.button("创建一条密码").clicked() {
                        self.current_page = Page::SubPage1;
                    }
                    if ui.button("查看所有密码").clicked() {
                        self.current_page = Page::SubPage2;
                    }
                    if ui.button("导入密码本").clicked() {
                        self.current_page = Page::SubPage3;
                    }
                    if ui.button("查找密码").clicked() {
                        self.current_page = Page::SubPage4;
                    }
                    if ui.button("修改密码本").clicked() {
                        self.current_page = Page::SubPage5;
                    }
                });
            },
            Page::SubPage1 => {
                egui::CentralPanel::default().show(ctx,|ui| {
                    ui.heading("创建密码");
                    ui.separator();

                    ui.label("请输入用户名");
                    ui.text_edit_singleline(&mut self.sub_page1data.input_name);

                    ui.label("请输入密码");
                    ui.text_edit_singleline(&mut self.sub_page1data.input_passwd);

                    ui.label("请输入备注");
                    ui.text_edit_singleline(&mut self.sub_page1data.input_notes);

                    if ui.button("确定").clicked() {
                        self.codebook.add(self.sub_page1data.input_passwd.clone(),
                                          self.sub_page1data.input_notes.clone(),
                                          self.sub_page1data.input_name.clone());

                        self.sub_page1data.input_passwd.clear();
                        self.sub_page1data.input_notes.clear();
                        self.sub_page1data.input_name.clear();
                        self.current_page = Page::MessagePage;
                    }

                });
            },
            Page::SubPage2 => {
                self.codebook.showdata();
                self.current_page = Page::MessagePage;
            },
            Page::SubPage3 => {

            },
            Page::SubPage4 => {
                egui::CentralPanel::default().show(ctx,|ui| {
                    ui.heading("找到密码数据");
                    ui.separator();

                    if ui.button("通过ID查找").clicked() {
                        self.current_page = Page::SubPage4_1;
                    }
                    if ui.button("通过名字查找").clicked() {
                        self.current_page = Page::SubPage4_2;
                    }
                    if ui.button("通过备注查找").clicked() {
                        self.current_page = Page::SubPage4_3;
                    }


                });
            },
            Page::SubPage4_1 =>{
                egui::CentralPanel::default().show(ctx,|ui|{
                    ui.heading("通过ID查找密码");
                    ui.separator();
                    ui.label("请输入ID");  //注意这里需要进行处理，只检查数字，其他过滤掉
                    egui_input_number(ui,&mut self.sub_page4_1data);


                    if self.sub_page4_1data.input_id != 0 {
                        if ui.button("确认").clicked(){
                            self.codebook.find(OptionType::ID(self.sub_page4_1data.input_id));
                            self.current_page = Page::MessagePage;
                            self.sub_page4_1data.input_id_str.clear();
                            self.sub_page4_1data.input_id = 0;
                        }
                    }

                });
            },
            Page::SubPage4_2 => {
                egui::CentralPanel::default().show(ctx,|ui|{
                    ui.heading("根据用户名 查找密码");
                    ui.separator();
                    ui.label("请输入用户名：");
                    ui.text_edit_singleline(&mut self.sub_page4_2data.input_name);

                    if self.sub_page4_2data.input_name.len() != 0 {
                        if ui.button("确认").clicked(){
                            self.codebook.find(OptionType::NAME(
                                self.sub_page4_2data.input_name.as_str()
                            ));
                            self.current_page = Page::MessagePage;
                            self.sub_page4_2data.input_name.clear();
                        }
                    }
                });
            },
            Page::SubPage4_3 => {
                egui::CentralPanel::default().show(ctx,|ui|{
                    ui.heading("根据备注 查找密码");
                    ui.separator();
                    ui.label("请输入备注：");
                    ui.text_edit_singleline(&mut self.sub_page4_3data.input_notes);

                    if self.sub_page4_3data.input_notes.len() != 0 {
                        if ui.button("确认").clicked(){
                            self.codebook.find(OptionType::NAME(
                                self.sub_page4_3data.input_notes.as_str()
                            ));
                            self.current_page = Page::MessagePage;
                            self.sub_page4_3data.input_notes.clear();
                        }
                    }
                });
            },
            Page::SubPage5 => {
                egui::CentralPanel::default().show(ctx,|ui| {
                    ui.heading("请选择操作方式");
                    ui.separator();

                    if ui.button("通过ID删除密码").clicked() {
                        self.current_page = Page::SubPage5_1;
                    }
                    if ui.button("通过ID修改密码").clicked() {
                        self.current_page = Page::SubPage5_2;
                    }
                });
            },
            Page::SubPage5_1 => {
                egui::CentralPanel::default().show(ctx,|ui|{
                    ui.heading("根据ID删除密码");
                    ui.separator();
                    ui.label("请输入ID");
                    egui_input_number(ui,&mut self.sub_page5_1data);
                    if self.sub_page5_1data.input_id != 0 {
                        if ui.button("确认").clicked(){
                            self.codebook.remove_by_id(self.sub_page5_1data.input_id);
                            self.current_page = Page::MessagePage;
                            self.sub_page5_1data.input_id = 0;
                            self.sub_page5_1data.input_id_str.clear();
                        }
                    }
                });
            },
            Page::SubPage5_2 => {
                egui::CentralPanel::default().show(ctx,|ui|{
                    ui.heading("根据ID修改密码数据");
                    ui.separator();
                    ui.label("请输入ID");
                    egui_input_number(ui,&mut self.sub_page5_2data.inputid);

                    if self.sub_page5_2data.inputid.input_id != 0 {
                        let mut nameopt = Option::<String>::None;
                        let mut passwdopt = Option::<String>::None;
                        let mut noteopt = Option::<String>::None;

                        ui.checkbox(&mut self.sub_page5_2data.input_name_status,"是否修改用户名");
                        if self.sub_page5_2data.input_name_status == true {
                            ui.label("请重新输入用户名：");
                            ui.text_edit_singleline(&mut self.sub_page5_2data.input_name);
                            if self.sub_page5_2data.input_name.len() != 0  {
                                nameopt = Option::Some(self.sub_page5_2data.input_name.clone());
                            }
                        }

                        ui.checkbox(&mut self.sub_page5_2data.input_passwd_status,"是否修改密码");
                        if self.sub_page5_2data.input_passwd_status == true {
                            ui.label("请重新输入密码：");
                            ui.text_edit_singleline(&mut self.sub_page5_2data.input_passwd);
                            if self.sub_page5_2data.input_passwd.len() != 0  {
                                passwdopt = Option::Some(self.sub_page5_2data.input_passwd.clone());
                            }

                        }

                        ui.checkbox(&mut self.sub_page5_2data.input_notes_status,"是否修改备注");
                        if self.sub_page5_2data.input_notes_status == true {
                            ui.label("请重新输入备注：");
                            ui.text_edit_singleline(&mut self.sub_page5_2data.input_notes);

                            if self.sub_page5_2data.input_notes.len() != 0 {
                                noteopt = Option::Some(self.sub_page5_2data.input_notes.clone());
                            }

                        }

                        if self.sub_page5_2data.input_notes.len() != 0 ||
                            self.sub_page5_2data.input_passwd.len() != 0 ||
                            self.sub_page5_2data.input_name.len() != 0 {
                                if ui.button("确认").clicked(){
                                    let ret = self.codebook.update_by_id(self.sub_page5_2data.inputid.input_id,
                                                                         nameopt,passwdopt,noteopt
                                    );
                                    match ret {
                                        Ok(_) => {

                                        },
                                        Err(e) => {
                                            info!("错误！！{}",e);
                                        },
                                    }
                                    self.current_page = Page::MessagePage;
                                    self.sub_page5_2data.clear();
                                }
                            }
                    }
                });
            },
            Page::MessagePage => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("执行结果");
                    ui.separator(); // 分隔线

                    egui::ScrollArea::vertical() // egui 框架中用于创建 ​垂直滚动区域 的 UI 组件
                        .id_source("log_scroll") // 必须的唯一ID
                        .auto_shrink([false; 2]) // 始终填充宽度
                        .stick_to_bottom(true)   // 自动滚动到底部
                        .show(ui, |ui| {
                            let logs = LOGS.lock();
                            for log in logs.iter() {
                                ui.label(egui::RichText::new(log)
                                             //.color(egui::Color32::GRAY) // 灰色文字
                                             //.text_style(egui::TextStyle::Monospace) // 等宽字体
                                );
                            }
                        });

                });
            },

        }

        if self.current_page != Page::Main {
            //TopBottomPanel 创建头部或尾部面板
            //TopBottomPanel::bottom 指明创建尾部面板
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx,|ui| {
                ui.horizontal(|ui| { //水平布局
                    if ui.button("← 返回主界面").clicked() {
                        self.current_page = Page::Main;
                        LOGS.lock().clear();
                    }
                });
            });
        }

    }
}


fn egui_input_number(ui: &mut egui::Ui,pagedata:&mut SubPage4_1Data) {
    let response = ui.add(egui::TextEdit::singleline(& mut pagedata.input_id_str));
    if response.changed() {
        let filtered = pagedata.input_id_str
            .chars()//这个会返回一个迭代器，按unicode字符遍历字符串，自动进行字符边界切割
            .enumerate() //获取字符及其位置索引（index,value）元组
            .filter(|(i,c)| { //进行过滤
                c.is_ascii_digit() //过滤数字
            })
            .map(|(_,c)|{c}) //将元组再打包，不要index，只要数据
            .collect();//将迭代器再转换为集合，这里将字符再转换为字符串

        pagedata.input_id_str = filtered;
        match pagedata.input_id_str.parse::<usize>() {
            Ok(value) => {
                pagedata.input_id = value;
            },
            Err(_) => {},
        }

    }
}

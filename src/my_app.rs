
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
    sub_page4data: SubPage1Data,

}

struct SubPage1Data {
    input_name :String,
    input_passwd:String,
    input_notes:String,
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
            sub_page4data: SubPage1Data::new(),
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
                    ui.label("请输入ID");
                });
            },
            Page::SubPage4_2 => {

            },
            Page::SubPage4_3 => {

            },
            Page::SubPage5 => {

            },
            Page::SubPage5_1 => {

            },
            Page::SubPage5_2 => {

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

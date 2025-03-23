#![windows_subsystem = "windows"]
use eframe::egui;
use std::path::Path;

#[derive(PartialEq,Debug)]
enum Page {
    Main,
    SubPage1,
    SubPage2,
    SubPage3,
    SubPage4,
}

struct MyApp {
    current_page: Page,  // 当前显示的页面
    subpage_data: String // 二级页面数据示例
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_page: Page::Main,
            subpage_data: String::new(),
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

        if self.current_page == Page::Main {
            egui::CentralPanel::default().show( //创建一个中央面板
                ctx,
                |ui| { //闭包中包含了面板中添加的UI元素
                    ui.heading("主界面"); //写了一个文本 主界面
                    ui.separator();//添加了一个分割线

                    // 添加了四个选项按钮，并描述了按钮被按下后的动作
                    if ui.button("选项 1").clicked() {
                        self.current_page = Page::SubPage1;
                    }
                    if ui.button("选项 2").clicked() {
                        self.current_page = Page::SubPage2;
                    }
                    if ui.button("选项 3").clicked() {
                        self.current_page = Page::SubPage3;
                    }
                    if ui.button("选项 4").clicked() {
                        self.current_page = Page::SubPage4;
                    }

            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(format!("二级界面 - {:?}", self.current_page));
                ui.separator();

                match self.current_page {
                    Page::SubPage1 => {
                        ui.label("这里是选项1的内容"); //这行代码是生产一个静态的文本显示，只读的
                        ui.text_edit_singleline(&mut self.subpage_data); //这行代码生成一个可输入文本框；是双向的，输入的值保存在subpage_data中，另外subpage_data如果在程序中被修改，文本框中的数据也会被修改

                    },
                    Page::SubPage2 => {
                        ui.label("这里是选项2的内容");
                        let mut length:usize = 0;
                        ui.add(egui::Slider::new(         //这里创建了一个滑块
                            &mut length,
                            0..=100 //滑块长度0到100
                        ));
                        ui.label(format!("Length: {}", length));
                    },
                    _ =>{
                        ui.label(format!("通用内容区域: {}", self.subpage_data));
                    },
                }
            });

            //TopBottomPanel 创建头部或尾部面板
            //TopBottomPanel::bottom 指明创建尾部面板
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx,|ui| {
                ui.horizontal(|ui| { //水平布局
                    if ui.button("← 返回主界面").clicked() {
                        self.current_page = Page::Main;
                        self.subpage_data.clear(); // 清除临时数据
                    }
                });
            });
        }

    }
}


fn main() {
    let options = eframe::NativeOptions::default(); //生产一个默认的窗口配置参数
    eframe::run_native(
        "GUI界面例子", //窗口名称
        options,// 传入窗口默认配置参数
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            #[cfg(target_os = "windows")]
            let font_path = "C:/Windows/Fonts/msyh.ttc"; // 微软雅黑

            // 检查字体是否存在
            if Path::new(font_path).exists() {
                fonts.font_data.insert(
                    "SystemFont".to_owned(),
                    egui::FontData::from_owned(std::fs::read(font_path).unwrap())
                );
                // 设置为默认字体
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                    .insert(0, "SystemFont".to_owned());
            } else {
                eprintln!("系统字体未找到，请安装中文字体！");
            }

            cc.egui_ctx.set_fonts(fonts);

            Box::new(MyApp::default())
        }
        ),//应用程序实例,使用闭包_cc里可以配置一些初始化参数，例如字体等
    ).unwrap(); //运行 GUI
}
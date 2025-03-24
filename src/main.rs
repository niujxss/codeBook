#![windows_subsystem = "windows"]

mod my_app;
mod code_book;
mod mat_data;

use eframe::egui;
use std::path::Path;
use crate::my_app::MyApp;




fn main() {
    let options = eframe::NativeOptions::default(); //生产一个默认的窗口配置参数
    eframe::run_native(
        "密码本", //窗口名称
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
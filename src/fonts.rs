use std::sync::Mutex;
use lazy_static::lazy_static;

// 添加图标缓存
lazy_static! {
    static ref FONTS: Mutex<Option<egui::FontDefinitions>> = Mutex::new(None);
}

pub fn init_fonts() -> egui::FontDefinitions {
    if FONTS.lock().unwrap().is_some() {
        return FONTS.lock().unwrap().clone().unwrap();
    }
    // 配置字体以支持中文
    let mut fonts = egui::FontDefinitions::default();

    // 添加支持中文的字体
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../resources/zh-cn.ttf")),
    );

    // 将字体设置为首选字体
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // 将字体设置为等宽字体
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());
    FONTS.lock().unwrap().replace(fonts.clone());
    fonts
}
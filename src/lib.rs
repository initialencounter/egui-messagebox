#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)]
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use eframe::egui;
use egui::IconData;

use fonts::init_fonts;
pub use icon::{load_icon_from_bytes, load_icon_from_path, load_icon_from_url};

mod fonts;
mod icon;

#[derive(Clone)]
pub struct DialogParams {
    pub title: String,
    pub content: String,
    pub confirm_button_text: String,
    pub cancel_button_text: String,
    pub options: eframe::NativeOptions,
}

unsafe impl Send for DialogParams {}

impl DialogParams {
    pub fn create(icon_data: Option<IconData>,
                  title: Option<String>,
                  content: Option<String>,
                  window_size: Option<[f32; 2]>,
                  confirm_button_text: Option<String>,
                  cancel_button_text: Option<String>,
    ) -> Self {
        let icon_data = icon_data.unwrap_or(IconData::default());
        let title = title.unwrap_or("Shit is on fire!".to_string());
        let content = content.unwrap_or("Shit is on fire!".to_string());
        let window_size = window_size.unwrap_or([320.0, 300.0]);
        let confirm_button_text = confirm_button_text.unwrap_or("Confirm".to_string());
        let cancel_button_text = cancel_button_text.unwrap_or("Cancel".to_string());
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(window_size)
                .with_icon(icon_data),
            ..Default::default()
        };
        Self {
            title,
            content,
            confirm_button_text,
            cancel_button_text,
            options,
        }
    }
}


struct InputDialog {
    heading_text: String,
    reply_text: String,
    dialog_params: DialogParams,
    tx: Sender<String>,
}

impl InputDialog {
    fn new(title: &str, content: &str, tx: Sender<String>, dialog_params: DialogParams) -> Self {
        Self {
            heading_text: title.to_owned(),
            reply_text: content.to_owned(),
            tx,
            dialog_params,
        }
    }
}

impl eframe::App for InputDialog {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.heading_text);

            ui.horizontal(|ui| {
                let name_label = ui.label("Input: ");
                ui.text_edit_singleline(&mut self.reply_text)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                if ui.button(&self.dialog_params.confirm_button_text).clicked() {
                    self.tx.send(self.reply_text.clone()).unwrap();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.button(&self.dialog_params.cancel_button_text).clicked() {
                    self.tx.send(String::new()).unwrap();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

struct ConfirmDialog {
    heading_text: String,
    tx: Sender<bool>,
    dialog_params: DialogParams,
}

impl ConfirmDialog {
    fn new(title: &str, tx: Sender<bool>, dialog_params: DialogParams) -> Self {
        Self {
            heading_text: title.to_owned(),
            tx,
            dialog_params,
        }
    }
}

impl eframe::App for ConfirmDialog {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.heading_text);


            ui.horizontal(|ui| {
                if ui.button(&self.dialog_params.confirm_button_text).clicked() {
                    self.tx.send(true).unwrap();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.button(&self.dialog_params.cancel_button_text).clicked() {
                    self.tx.send(false).unwrap();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

pub fn show_dialog<T: eframe::App + 'static>(dialog_params: DialogParams, dialog: T) {
    let _ = eframe::run_native(
        &dialog_params.title,
        dialog_params.options,
        Box::new(|cc| {
            let fonts = init_fonts();
            cc.egui_ctx.set_fonts(fonts);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(dialog))
        }),
    );
}
pub async fn confirm(dialog_params: DialogParams) -> bool {
    let (tx, rx) = mpsc::channel::<bool>();
    let dialog_params_clone = dialog_params.clone();
    let dialog = ConfirmDialog::new(&dialog_params.content, tx, dialog_params_clone);
    show_dialog::<ConfirmDialog>(dialog_params, dialog);
    match rx.try_recv() {
        Ok(result) => result,
        Err(e) => {
            println!("error: {}", e);
            false
        },
    }
}

pub async fn input(dialog_params: DialogParams) -> String {
    let (tx, rx) = mpsc::channel::<String>();
    let dialog_params_clone = dialog_params.clone();
    let dialog = InputDialog::new(&dialog_params.content, "ok", tx, dialog_params_clone);
    show_dialog::<InputDialog>(dialog_params, dialog);
    match rx.try_recv() {
        Ok(result) => result,
        Err(e) => {
            println!("error: {}", e);
            String::new()
        },
    }
}

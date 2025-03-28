#![feature(let_chains)]

use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, Ui};
use eframe::{Frame, egui};
use rust_tool_terminal_command_tips::command::Command;
use std::collections::HashMap;

struct TerminalTips {
    command_args_map: HashMap<usize, HashMap<String, String>>,
}

impl Default for TerminalTips {
    fn default() -> Self {
        TerminalTips {
            command_args_map: HashMap::new(),
        }
    }
}

impl TerminalTips {
    fn draw_command(
        &mut self,
        ui: &mut Ui,
        commands: &mut Vec<Command>,
    ) -> HashMap<u8, HashMap<String, String>> {
        let result = HashMap::new();
        for x in 0..commands.len() {
            let mut command = &commands[x];
            // let mut command_args_map: HashMap<K, V> = HashMap::new();
            let command_split = command.command.split_whitespace();
            let command_slice = command_split.collect::<Vec<&str>>();

            let command_arg_map = self.command_args_map.entry(x).or_insert(HashMap::new());

            // ui.label(command.to_string());
            ui.horizontal(|ui| {
                ui.label(&command.command);
                for x in &command.args {
                    ui.label(&x.description);
                }
                ui.label(&command.description);
            });

            ui.horizontal(|ui| {
                for command_sli in command_slice {
                    if command_sli.starts_with("[") && command_sli.ends_with("]") {
                        let command_sli = command_sli.replace("[", "").replace("]", "");
                        let a = command_arg_map
                            .entry(command_sli.to_string())
                            .or_insert(String::new());
                        ui.text_edit_singleline(a);
                    }
                }
                if ui.button("生成命令").clicked() {
                    if command_arg_map.iter().any(|(key, value)| value.is_empty()) {
                        println!("有值为空");
                    } else {
                        println!("空检测通过");
                        let mut full_command = command.command.clone();
                        command.args.iter().for_each(|arg| {
                            if let Some(value) = command_arg_map.get(&arg.name) {
                                full_command = full_command
                                    .replace(arg.name.as_str(), value)
                                    .replace("[", "")
                                    .replace("]", "");
                            }
                        });
                        println!("full:{}", full_command);
                        ui.ctx().copy_text(full_command);
                    }
                }
            });
            // self.command_args_map.insert(x, command_args_map);
        }
        result
    }
}

impl eframe::App for TerminalTips {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            init_config(ctx, ui);
            ui.vertical(|ui| {
                let mut commands = Command::load_commands();
                // ui.text_edit_singleline(&mut self.name);
                self.draw_command(ui, &mut commands);
            })
        });
    }
}

fn init_config(ctx: &Context, ui: &mut egui::Ui) {
    if let Ok(title) = std::env::var("TITLE") {
        ui.heading(title);
    } else {
        ui.heading("终端命令提示工具");
    }
    font_set(ctx);
}

fn font_set(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "my_font".to_owned(),
        std::sync::Arc::new(
            // .ttf and .otf supported
            FontData::from_static(include_bytes!("../media/fonts/DroidSansFallbackFull.ttf")),
        ),
    );

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024f32, 720f32]),
        ..Default::default()
    };
    eframe::run_native(
        "terminal command tips",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<TerminalTips>::default())
        }),
    );
}

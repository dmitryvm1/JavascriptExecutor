use egui::TextBuffer;

use crate::{
    app_state::AppState,
    code_editor::{CodeEditor, View},
    js_runner::{self, JsRunnerComm},
    persistent_state::PersistentState,
};

pub struct MyApp {
    ui_state: PersistentState<AppState>,
    code_editor: CodeEditor,
    output: String,
    js_runner: JsRunnerComm,
}

/// Immutable string buffer
pub struct LockedStr<'a> {
    pub s: &'a str,
}

impl<'a> TextBuffer for LockedStr<'a> {
    fn char_range(&self, char_range: std::ops::Range<usize>) -> &str {
        &self.s[char_range.start..char_range.end]
    }

    fn byte_index_from_char_index(&self, char_index: usize) -> usize {
        self.as_str().byte_index_from_char_index(char_index)
    }

    fn clear(&mut self) {}

    fn replace(&mut self, _text: &str) {}

    fn take(&mut self) -> String {
        let s = self.s.to_owned();
        self.clear();
        s
    }

    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        self.s
    }

    fn insert_text(&mut self, _text: &str, _char_index: usize) -> usize {
        0
    }

    fn delete_char_range(&mut self, _char_range: std::ops::Range<usize>) {}
}

impl Default for MyApp {
    fn default() -> Self {
        let ui_state = PersistentState::<AppState>::default();
        let js_runner = js_runner::spawn();
        Self {
            output: String::with_capacity(1024),
            code_editor: CodeEditor::default(),
            ui_state,
            js_runner,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(r) = self.js_runner.peek_result() {
            self.output.push_str(&r);
            self.output.push_str("\n");
        }
        self.ui(ctx);
    }
}

impl MyApp {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("output").resizable(true).max_height(200.0).default_height(150.0).show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::TOP).with_cross_justify(true),
                |ui| {
                    ui.label("Output:");
                    egui::ScrollArea::vertical()
                        .max_height(190.0)
                        .show(ui, |ui| {
                            egui::TextEdit::multiline(&mut LockedStr { s: &self.output })
                            .desired_width(f32::INFINITY)
                            .show(ui);
                        });
                },
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Javascript Runner:");
            ui.separator();
            egui::Resize::default().max_size([790.0, 300.0]).default_size([790.0, 300.0]).show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP).with_cross_justify(true),
                    |ui| {
                        self.code_editor.ui(ui);
                    });
            });
            ui.horizontal(|ui| {
                if ui.button("Run").clicked() {
                    self.js_runner
                        .enqueue_script(self.code_editor.code.as_str())
                };
            });
        });
    }
}

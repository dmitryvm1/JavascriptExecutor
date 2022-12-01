use super::syntax_highlighting::{CodeTheme, self};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CodeEditor {
    language: String,
    pub cursor_at: usize,
    pub code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            cursor_at: 0,
            language: "js".into(),
            code: r#"1 + 1"#.into(),
        }
    }
}

/// Something to view
pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

}


impl Demo for CodeEditor {
    fn name(&self) -> &'static str {
        "🖮 Code Editor"
    }

}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl View for CodeEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { language, code, cursor_at: _ } = self;
        let theme = CodeTheme::from_memory(ui.ctx());
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };

        egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
            let w = egui::TextEdit::multiline(code)
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(10)
            .lock_focus(true)
            .desired_width(f32::INFINITY)
            .layouter(&mut layouter);
            // let output = ui.add(w);
            let output = w.show(ui);
            if let Some(cu) = output.cursor_range {
                self.cursor_at = cu.primary.ccursor.index;
            }
        });
        
    }
}
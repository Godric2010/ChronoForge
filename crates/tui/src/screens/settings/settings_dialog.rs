use crate::app_action::AppAction;
use crate::screens::dialog::{Dialog, DialogResult};
use crate::widgets::dialog_widgets::text_input::TextInputWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub enum SettingsDialogResult {
    None,
    Cancelled,
    Confirmed(AppAction),
}

pub enum SettingsDialog {
    ImportCsv(Dialog<TextInputWidget>),
    ExportCSV(Dialog<TextInputWidget>),
}

impl SettingsDialog {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            SettingsDialog::ImportCsv(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::ExportCSV(dialog) => {
                dialog.render(frame, area);
            }
        }
    }

    pub fn handle_input(&mut self, event: KeyEvent) -> SettingsDialogResult {
        match self {
            SettingsDialog::ImportCsv(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::ImportCsv(result))
                    }
                }
            }
            SettingsDialog::ExportCSV(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::ExportCsv(result))
                    }
                }
            }
        }
    }
}

use crate::screens::dialog::Dialog;
use crate::screens::settings::settings_dialog::SettingsDialog;
use crate::widgets::dialog_widgets::text_input::TextInputWidget;

#[allow(dead_code)]
pub enum SettingsActionTarget {
    DemoToggle,
    DemoValue,
}

pub enum SettingsActionPurpose {
    ImportCSV,
    ExportCSV,
}

impl SettingsActionPurpose {
    pub fn build(&self) -> SettingsDialog {
        match self {
            SettingsActionPurpose::ImportCSV => {
                let widget = TextInputWidget::new();
                let dialog = Dialog::<TextInputWidget>::new("Set path to import CSV from", widget);
                SettingsDialog::ImportCsv(dialog)
            }
            SettingsActionPurpose::ExportCSV => {
                let widget = TextInputWidget::new();
                let dialog = Dialog::<TextInputWidget>::new("Set path to export CSV", widget);
                SettingsDialog::ExportCSV(dialog)
            }
        }
    }
}

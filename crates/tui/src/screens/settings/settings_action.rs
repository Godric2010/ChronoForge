use crate::screens::dialog::Dialog;
use crate::screens::settings::settings_dialog::SettingsDialog;
use crate::widgets::dialog_widgets::PathWidget;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleActionTarget {
    ShowArchivedProjects,
    ShowArchivedTasks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeTargetAction {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub enum SettingsActionPurpose {
    ImportCSV,
    ExportCSV,
    LinkNewDatabase,
    MoveDatabase,
}

impl SettingsActionPurpose {
    pub fn build(&self) -> SettingsDialog {
        match self {
            SettingsActionPurpose::ImportCSV => {
                let widget = PathWidget::new();
                let dialog = Dialog::new("Set path to import CSV from", widget);
                SettingsDialog::ImportCsv(dialog)
            }
            SettingsActionPurpose::ExportCSV => {
                let widget = PathWidget::new();
                let dialog = Dialog::new("Set path to export CSV", widget);
                SettingsDialog::ExportCSV(dialog)
            }
            SettingsActionPurpose::LinkNewDatabase => {
                let widget = PathWidget::new();
                let dialog = Dialog::new("Link new database", widget);
                SettingsDialog::LinkNewDatabase(dialog)
            }
            SettingsActionPurpose::MoveDatabase => {
                let widget = PathWidget::new();
                let dialog = Dialog::new("Move database", widget);
                SettingsDialog::MoveDatabase(dialog)
            }
        }
    }
}

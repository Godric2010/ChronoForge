use crate::app_action::AppAction;
use crate::input::help_context::InputMapHelpContext;
use crate::screens::dialog::{Dialog, DialogResult};
use crate::widgets::dialog_widgets::{PathWidget, TimeWidget};
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::path::PathBuf;

pub enum SettingsDialogResult {
    None,
    Cancelled,
    Confirmed(AppAction),
    Help(Vec<InputMapHelpContext>),
}

pub enum SettingsDialog {
    ImportCsv(Dialog<PathWidget>),
    ExportCSV(Dialog<PathWidget>),
    LinkNewDatabase(Dialog<PathWidget>),
    MoveDatabase(Dialog<PathWidget>),
    SetMondayWorkTarget(Dialog<TimeWidget>),
    SetTuesdayWorkTarget(Dialog<TimeWidget>),
    SetWednesdayWorkTarget(Dialog<TimeWidget>),
    SetThursdayWorkTarget(Dialog<TimeWidget>),
    SetFridayWorkTarget(Dialog<TimeWidget>),
    SetSaturdayWorkTarget(Dialog<TimeWidget>),
    SetSundayWorkTarget(Dialog<TimeWidget>),
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
            SettingsDialog::SetMondayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::SetTuesdayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::SetWednesdayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::SetThursdayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::SetFridayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::SetSaturdayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::SetSundayWorkTarget(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::LinkNewDatabase(dialog) => {
                dialog.render(frame, area);
            }
            SettingsDialog::MoveDatabase(dialog) => {
                dialog.render(frame, area);
            }
        }
    }

    pub fn handle_input(&mut self, event: KeyEvent) -> SettingsDialogResult {
        match self {
            SettingsDialog::LinkNewDatabase(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        let path = PathBuf::from(result);
                        if !path.parent().unwrap().exists() {
                            return SettingsDialogResult::Cancelled;
                        }
                        SettingsDialogResult::Confirmed(AppAction::LinkNewDatabase(path))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::MoveDatabase(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        let path = PathBuf::from(result);
                        if !path.parent().unwrap().exists() {
                            return SettingsDialogResult::Cancelled;
                        }
                        SettingsDialogResult::Confirmed(AppAction::MoveDatabase(path))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::ImportCsv(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::ImportCsv(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
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
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetMondayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetMondayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetTuesdayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetTuesdayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetWednesdayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetWednesdayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetThursdayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetThursdayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetFridayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetFridayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetSaturdayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetSaturdayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
            SettingsDialog::SetSundayWorkTarget(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => SettingsDialogResult::None,
                    DialogResult::Cancelled => SettingsDialogResult::Cancelled,
                    DialogResult::Confirmed(result) => {
                        SettingsDialogResult::Confirmed(AppAction::SetSundayWorkTarget(result))
                    }
                    DialogResult::Help(help_context) => SettingsDialogResult::Help(help_context),
                }
            }
        }
    }
}

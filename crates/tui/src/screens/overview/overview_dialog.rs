use crate::app_action::AppAction;
use crate::screens::dialog::{Dialog, DialogResult};
use crate::widgets::dialog_widgets::{ListWidget, TextInputWidget, TimeEntryWidget, YesNoWidget};
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;
use uuid::Uuid;

pub enum OverviewDialogResult {
    None,
    Cancelled,
    Confirmed(AppAction),
}

pub enum OverviewDialog {
    CreateProject(Dialog<TextInputWidget>),
    EditProjectName(Dialog<TextInputWidget>, Uuid),
    DeleteProject(Dialog<YesNoWidget>, Uuid),

    CreateTask(Dialog<TextInputWidget>, Uuid),
    EditTask(Dialog<TextInputWidget>, Uuid),
    AssignTask(Dialog<ListWidget>, Uuid),
    DeleteTask(Dialog<YesNoWidget>, Uuid),

    CreateTimeEntry(Dialog<TimeEntryWidget>, Uuid),
    EditTimeEntry(Dialog<TimeEntryWidget>, Uuid),
    AssignTimeEntry(Dialog<ListWidget>, Uuid),
    DeleteTimeEntry(Dialog<YesNoWidget>, Uuid),
}

impl OverviewDialog {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            OverviewDialog::CreateProject(dialog) => {
                dialog.render(frame, area);
            }
            OverviewDialog::EditProjectName(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::CreateTask(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::EditTask(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::DeleteProject(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::DeleteTask(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::DeleteTimeEntry(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::AssignTask(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::AssignTimeEntry(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::CreateTimeEntry(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::EditTimeEntry(dialog, _) => {
                dialog.render(frame, area);
            }
        }
    }

    pub fn handle_input(&mut self, event: KeyEvent) -> OverviewDialogResult {
        match self {
            OverviewDialog::CreateProject(dialog) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(project_name) => {
                        OverviewDialogResult::Confirmed(AppAction::CreateProject(project_name))
                    }
                }
            }
            OverviewDialog::EditProjectName(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(new_project_name) => OverviewDialogResult::Confirmed(
                        AppAction::RenameProject(project_id.clone(), new_project_name),
                    ),
                }
            }
            OverviewDialog::CreateTask(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(task_name) => OverviewDialogResult::Confirmed(
                        AppAction::CreateTask(task_name, project_id.clone()),
                    ),
                }
            }
            OverviewDialog::EditTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(new_task_name) => OverviewDialogResult::Confirmed(
                        AppAction::RenameTask(task_id.clone(), new_task_name),
                    ),
                }
            }
            OverviewDialog::DeleteProject(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(deletion_confirmed) => {
                        if deletion_confirmed {
                            OverviewDialogResult::Confirmed(AppAction::DeleteProject(
                                project_id.clone(),
                            ))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                }
            }
            OverviewDialog::DeleteTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(deletion_confirmed) => {
                        if deletion_confirmed {
                            OverviewDialogResult::Confirmed(AppAction::DeleteTask(task_id.clone()))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                }
            }
            OverviewDialog::DeleteTimeEntry(dialog, entry_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(deletion_confirmed) => {
                        if deletion_confirmed {
                            OverviewDialogResult::Confirmed(AppAction::DeleteTimeEntry(
                                entry_id.clone(),
                            ))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                }
            }
            OverviewDialog::AssignTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(project_id) => OverviewDialogResult::Confirmed(
                        AppAction::AssignTask(task_id.clone(), project_id.clone()),
                    ),
                }
            }
            OverviewDialog::AssignTimeEntry(dialog, entry_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(task_id) => OverviewDialogResult::Confirmed(
                        AppAction::AssignTimeEntry(entry_id.clone(), task_id.clone()),
                    ),
                }
            }
            OverviewDialog::CreateTimeEntry(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(times) => {
                        if let Some(times) = times {
                            OverviewDialogResult::Confirmed(AppAction::CreateTimeEntry(
                                task_id.clone(),
                                times.0,
                                times.1,
                            ))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                }
            }
            OverviewDialog::EditTimeEntry(dialog, entry_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(times) => {
                        if let Some(times) = times {
                            OverviewDialogResult::Confirmed(AppAction::EditTimeEntry(
                                entry_id.clone(),
                                times.0,
                                times.1,
                            ))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                }
            }
        }
    }
}

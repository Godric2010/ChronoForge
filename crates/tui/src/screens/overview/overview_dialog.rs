use crate::app_action::AppAction;
use crate::input::help_context::InputMapHelpContext;
use crate::screens::dialog::{Dialog, DialogResult};
use crate::widgets::dialog_widgets::{
    ListWidget, ProjectEditWidget, TaskEditWidget, TimeEntryWidget, YesNoWidget,
};
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;
use uuid::Uuid;

pub enum OverviewDialogResult {
    None,
    Cancelled,
    Confirmed(AppAction),
    Help(Vec<InputMapHelpContext>),
}

pub enum OverviewDialog {
    CreateProject(Dialog<ProjectEditWidget>),
    EditProjectName(Dialog<ProjectEditWidget>, Uuid),
    DeleteProject(Dialog<YesNoWidget>, Uuid),
    ArchiveProject(Dialog<YesNoWidget>, Uuid),
    UnarchiveProject(Dialog<YesNoWidget>, Uuid),

    CreateTask(Dialog<TaskEditWidget>, Uuid),
    EditTask(Dialog<TaskEditWidget>, Uuid),
    AssignTask(Dialog<ListWidget>, Uuid),
    DeleteTask(Dialog<YesNoWidget>, Uuid),
    ArchiveTask(Dialog<YesNoWidget>, Uuid),
    UnarchiveTask(Dialog<YesNoWidget>, Uuid),

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
            OverviewDialog::ArchiveProject(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::ArchiveTask(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::UnarchiveProject(dialog, _) => {
                dialog.render(frame, area);
            }
            OverviewDialog::UnarchiveTask(dialog, _) => {
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
                    DialogResult::Confirmed(output) => OverviewDialogResult::Confirmed(
                        AppAction::CreateProject(output.project_name, output.time_limit),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::EditProjectName(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(output) => OverviewDialogResult::Confirmed(
                        AppAction::EditProject(*project_id, output.project_name, output.time_limit),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::CreateTask(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(output) => OverviewDialogResult::Confirmed(
                        AppAction::CreateTask(output.task_name, output.time_limit, *project_id),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::EditTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(output) => OverviewDialogResult::Confirmed(
                        AppAction::RenameTask(*task_id, output.task_name, output.time_limit),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::DeleteProject(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(deletion_confirmed) => {
                        if deletion_confirmed {
                            OverviewDialogResult::Confirmed(AppAction::DeleteProject(*project_id))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::DeleteTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(deletion_confirmed) => {
                        if deletion_confirmed {
                            OverviewDialogResult::Confirmed(AppAction::DeleteTask(*task_id))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::DeleteTimeEntry(dialog, entry_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(deletion_confirmed) => {
                        if deletion_confirmed {
                            OverviewDialogResult::Confirmed(AppAction::DeleteTimeEntry(*entry_id))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::AssignTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(project_id) => {
                        OverviewDialogResult::Confirmed(AppAction::AssignTask(*task_id, project_id))
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::AssignTimeEntry(dialog, entry_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(task_id) => OverviewDialogResult::Confirmed(
                        AppAction::AssignTimeEntry(*entry_id, task_id),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::CreateTimeEntry(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(times) => OverviewDialogResult::Confirmed(
                        AppAction::CreateTimeEntry(*task_id, times.0, times.1),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::EditTimeEntry(dialog, entry_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(times) => OverviewDialogResult::Confirmed(
                        AppAction::EditTimeEntry(*entry_id, times.0, times.1),
                    ),
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::ArchiveProject(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(archive_project) => {
                        if archive_project {
                            OverviewDialogResult::Confirmed(AppAction::ArchiveProject(*project_id))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::ArchiveTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(archive_task) => {
                        if archive_task {
                            OverviewDialogResult::Confirmed(AppAction::ArchiveTask(*task_id))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::UnarchiveProject(dialog, project_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(unarchive_project) => {
                        if unarchive_project {
                            OverviewDialogResult::Confirmed(AppAction::UnarchiveProject(
                                *project_id,
                            ))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
            OverviewDialog::UnarchiveTask(dialog, task_id) => {
                let result = dialog.handle_input(event);
                match result {
                    DialogResult::None => OverviewDialogResult::None,
                    DialogResult::Cancelled => OverviewDialogResult::Cancelled,
                    DialogResult::Confirmed(unarchive_task) => {
                        if unarchive_task {
                            OverviewDialogResult::Confirmed(AppAction::UnarchiveTask(*task_id))
                        } else {
                            OverviewDialogResult::Cancelled
                        }
                    }
                    DialogResult::Help(help_context) => OverviewDialogResult::Help(help_context),
                }
            }
        }
    }
}

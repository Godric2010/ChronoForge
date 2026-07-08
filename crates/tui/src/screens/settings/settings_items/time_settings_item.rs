use crate::screens::dialog::Dialog;
use crate::screens::settings::settings_action::TimeTargetAction;
use crate::screens::settings::settings_dialog::SettingsDialog;
use crate::widgets::dialog_widgets::TimeWidget;
use domain::types::UserSettings;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct TimeSettingsItem {
    name: String,
    description: String,
    value: u32,
    target: TimeTargetAction,
}

impl TimeSettingsItem {
    pub fn new(name: &str, description: &str, value: u32, target: TimeTargetAction) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            value,
            target,
        }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        name_area: Rect,
        value_area: Rect,
        description_area: Rect,
        style: Style,
    ) {
        let name_paragraph = Paragraph::new(self.name.clone()).style(style);
        frame.render_widget(name_paragraph, name_area);

        let value_string = format!("{:02}h {:02}min", self.value / 60, self.value % 60);
        let value_paragraph = Paragraph::new(value_string).style(style);
        frame.render_widget(value_paragraph, value_area);

        let description_paragraph =
            Paragraph::new(self.description.clone()).style(style.add_modifier(Modifier::ITALIC));
        frame.render_widget(description_paragraph, description_area);
    }

    pub fn update(&mut self, settings: &UserSettings) {
        self.value = match self.target {
            TimeTargetAction::Monday => settings.monday_target_minutes,
            TimeTargetAction::Tuesday => settings.tuesday_target_minutes,
            TimeTargetAction::Wednesday => settings.wednesday_target_minutes,
            TimeTargetAction::Thursday => settings.thursday_target_minutes,
            TimeTargetAction::Friday => settings.friday_target_minutes,
            TimeTargetAction::Saturday => settings.saturday_target_minutes,
            TimeTargetAction::Sunday => settings.sunday_target_minutes,
        }
    }

    pub fn execute_action(&self) -> SettingsDialog {
        let widget = TimeWidget::new(self.value);
        match self.target {
            TimeTargetAction::Monday => {
                let dialog = Dialog::new("Set Monday Work Target", widget);
                SettingsDialog::SetMondayWorkTarget(dialog)
            }
            TimeTargetAction::Tuesday => {
                let dialog = Dialog::new("Set Tuesday Work Target", widget);
                SettingsDialog::SetTuesdayWorkTarget(dialog)
            }
            TimeTargetAction::Wednesday => {
                let dialog = Dialog::new("Set Wednesday Work Target", widget);
                SettingsDialog::SetWednesdayWorkTarget(dialog)
            }
            TimeTargetAction::Thursday => {
                let dialog = Dialog::new("Set Thrusday Work Target", widget);
                SettingsDialog::SetThursdayWorkTarget(dialog)
            }
            TimeTargetAction::Friday => {
                let dialog = Dialog::new("Set Friday Work Target", widget);
                SettingsDialog::SetFridayWorkTarget(dialog)
            }
            TimeTargetAction::Saturday => {
                let dialog = Dialog::new("Set Saturday Work Target", widget);
                SettingsDialog::SetSaturdayWorkTarget(dialog)
            }
            TimeTargetAction::Sunday => {
                let dialog = Dialog::new("Set Sunday Work Target", widget);
                SettingsDialog::SetSundayWorkTarget(dialog)
            }
        }
    }
}

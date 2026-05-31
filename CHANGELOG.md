# Changelog

## [0.1.1] 2026-05-31
### Bugfixes
- Trying to decrease a time entry value below zero does not crash the program anymore
- Invalid time entry because of end time before start time is now validated after editing start and end time alike
- Failing validation for the end time does now trigger the correct error message 
- Move list index up and down in list dialog widget when the list is empty does not crash the program anymore
- Requesting the output in list dialog widget when the list is empty does not crash the program anymore
- Invalid input in text input dialog (e.g. everything that is not ASCII) does not crash the program anymore
- Moving list index up and down in card lists when the list is empty does not crash the program anymore
- Invalid paths when importing or exporting csv files does not crash the program anymore
- Invalid csv files during import do not crash the program anymore

### Changes
- Failed actions are now displayed as in-app error dialogs
- Time Entry Dialog now supports BackTab to move backwards through the digit edit fields
- Introduced new app error type
- Introduced new ui error message and dialog window
- Modified the dialog widget trait to allow for more versatile help texts and different colors
- Added tests for input validation in dialog widgets
- Added tests for input in card list widget
- Added colorized border for dialogs by default

## [0.1.0] - 2026-05-28
- Initial pre-release version
- Added project management
  - Create, rename, delete projects via TUI / CLI
- Added task management
  - Create, rename, delete tasks via TUI / CLI
  - Re-assign tasks to other projects via TUI / CLI
- Added time entry management
  - Create, edit, delete time entries via TUI / CLI
  - Re-assign time entries to other tasks via TUI / CLI
- Start/Stop time tracking for a task via TUI / CLI
- Export database as csv via TUI
- Import database from csv via TUI

### Known bugs
- Invalid paths on Importing/Exporting csv file result in crash
- Incompatible csv imports result in crash
- In time entry edit, reducing any number lower than 0 results in crash
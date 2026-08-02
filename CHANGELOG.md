# Changelog

## [0.2.0] - 2026-08-02
### Changes
- Added optional target times for projects and tasks
  - Projects and tasks now store their target time as optional values
  - Target times can be edited via CLI and TUI
  - Project and task cards now display worked time and target time when a target time is set
  - Worked time is highlighted when the target time is exceeded
- Added new edit widgets for projects and tasks
  - Added reusable text edit, time edit, date edit and checkbox elements
  - Replaced the old text input widget with the new text edit element
  - Refactored project, task and time entry dialogs to use the new input elements
- Refactored key binding and input handling
  - Added centralized key binding and input map types
  - Screens, dialogs and input elements now use shared input maps
  - Added context-sensitive footer help derived from input maps
  - Added help dialogs for overview, settings and dialogs
- Added project and task archiving
  - Projects and tasks can now be archived and unarchived
  - Archived projects and tasks can be included or hidden through filters
  - Added CLI commands to archive and unarchive projects and tasks
  - Added CLI flags to list archived projects and tasks
  - Added TUI key bindings to archive and unarchive projects and tasks from the overview
  - Archived projects and tasks are marked in the project and task cards
- Added user settings
  - Added persistent settings for showing archived projects and tasks
  - Added persistent daily work time targets for each weekday
  - Added settings screen items to toggle archived project/task visibility
  - Added settings screen items to edit daily work time targets
- Improved time visualization
  - Daily time visualization now shows elapsed time for the current day and the configured target work time
- Added app configuration support
  - Added platform-specific config file loading at application startup
  - Added settings view model to display app configuration values
  - Added support for moving the current database
  - Added support for linking ChronoForge to an existing database
  - The current database path is now shown in the settings screen
- Added startup setup flow
  - TUI now opens a setup screen when no config or database exists
  - Setup screen can create a new database, link an existing database or quit the app
  - CLI creates a new database automatically when no config exists
- Added path validation
  - Path widgets now return PathBuf
  - CSV import/export paths are validated before execution
  - Database create/link/move paths are validated before execution

### Bugfixes
- Improved compatibility with older CSV files where target times are not present yet
- Fixed time entry card date display when a time entry is older than one week
- Fixed dialogs not showing correctly for time entries
- Fixed wrong weekday handling for time entries caused by local time vs. UTC conversion
- Prevented the TUI from silently continuing when the configured database is missing
- Invalid paths for CSV and database operations are now blocked before execution
- Removed clippy warnings and cleaned up minor idiomatic Rust issues

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
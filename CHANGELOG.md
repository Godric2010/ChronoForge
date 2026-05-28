# Changelog

## [0.1.0] - 2026-06-28
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
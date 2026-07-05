ALTER TABLE projects
ADD COLUMN is_archived INTEGER NOT NULL DEFAULT 0;

ALTER TABLE tasks
ADD COLUMN is_archived INTEGER NOT NULL DEFAULT 0;

CREATE TABLE IF NOT EXISTS user_settings(
    id INTEGER PRIMARY KEY CHECK ( id = 1 ),

    monday_target_minutes INTEGER NOT NULL DEFAULT 480,
    tuesday_target_minutes INTEGER NOT NULL DEFAULT 480,
    wednesday_target_minutes INTEGER NOT NULL DEFAULT 480,
    thursday_target_minutes INTEGER NOT NULL DEFAULT 480,
    friday_target_minutes INTEGER NOT NULL DEFAULT 480,
    saturday_target_minutes INTEGER NOT NULL DEFAULT 0,
    sunday_target_minutes INTEGER NOT NULL DEFAULT 0,

    show_archived_projects INTEGER NOT NULL DEFAULT 0,
    show_archived_tasks INTEGER NOT NULL DEFAULT 0,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO user_settings (id)
VALUES (1);
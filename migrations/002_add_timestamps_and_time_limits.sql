ALTER TABLE projects ADD COLUMN time_limit INTEGER;
ALTER TABLE projects ADD COLUMN created_at TEXT;
ALTER TABLE projects ADD COLUMN updated_at TEXT;


UPDATE projects
SET
    time_limit = COALESCE(time_limit, 0),
    created_at = COALESCE(created_at, CURRENT_TIMESTAMP),
    updated_at = COALESCE(updated_at, CURRENT_TIMESTAMP);

ALTER TABLE tasks ADD COLUMN time_limit INTEGER;
ALTER TABLE tasks ADD COLUMN created_at TEXT;
ALTER TABLE tasks ADD COLUMN updated_at TEXT;

UPDATE tasks
SET
    time_limit = COALESCE(time_limit, 0),
    created_at = COALESCE(created_at, CURRENT_TIMESTAMP),
    updated_at = COALESCE(updated_at, CURRENT_TIMESTAMP);

ALTER TABLE time_entries ADD COLUMN created_at TEXT;
ALTER TABLE time_entries ADD COLUMN updated_at TEXT;

UPDATE time_entries
SET
    created_at = COALESCE(created_at, CURRENT_TIMESTAMP),
    updated_at = COALESCE(updated_at, CURRENT_TIMESTAMP);
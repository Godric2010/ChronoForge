PRAGMA
foreign_keys = ON;

CREATE TABLE projects
(
    id   TEXT PRIMARY KEY NOT NULL,
    name TEXT             NOT NULL
);

CREATE TABLE tasks
(
    id         TEXT PRIMARY KEY NOT NULL,
    project_id TEXT             NOT NULL,
    name       TEXT             NOT NULL,

    FOREIGN KEY (project_id)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE TABLE time_entries
(
    id         TEXT PRIMARY KEY NOT NULL,
    task_id    TEXT             NOT NULL,
    start_time TEXT             NOT NULL,
    end_time   TEXT             NOT NULL,

    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
        ON DELETE CASCADE
);

CREATE TABLE active_timer
(
    id         INTEGER PRIMARY KEY CHECK ( id = 1 ),
    task_id    TEXT NOT NULL,
    start_time TEXT NOT NULL,

    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
        ON DELETE CASCADE
);
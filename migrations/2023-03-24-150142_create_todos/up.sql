PRAGMA foreign_keys = OFF;

CREATE TABLE todos_new(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(20) NOT NULL,
    done BOOLEAN default 0,
    created_by INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(created_by) REFERENCES users(id)
);

INSERT INTO todos_new(id, title, done, created_by, created_at, updated_at)
SELECT id, title, done, null, created_at, created_at FROM todos;

DROP TABLE todos;

ALTER TABLE todos_new RENAME TO todos;

PRAGMA foreign_keys = ON;


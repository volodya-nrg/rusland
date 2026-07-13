CREATE TABLE orders
(
    order_id    CHAR(36) PRIMARY KEY, -- кастомный uuid
    fio         VARCHAR(255),
    tel         VARCHAR(255)                                    NOT NULL,
    email       VARCHAR(255),
    description TEXT,
    created_at  DATETIME DEFAULT (datetime('now', 'localtime')) NOT NULL
);
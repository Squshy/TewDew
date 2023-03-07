CREATE TABLE tasks (
    id uuid NOT NULL,
    tewdew_id uuid NOT NULL,
    user_id uuid NOT NULL,
    completed boolean NOT NULL,
    title TEXT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_tewdew FOREIGN KEY(tewdew_id) REFERENCES tewdews(id),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);

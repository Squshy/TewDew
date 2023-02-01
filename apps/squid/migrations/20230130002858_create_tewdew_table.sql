-- Create tewdews table which references users
CREATE TABLE tewdews(
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    completed boolean NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

ALTER TABLE tewdews ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE tewdews ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
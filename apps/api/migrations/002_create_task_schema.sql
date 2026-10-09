-- Task Schema
CREATE TABLE app.tasks (
    id uuid PRIMARY KEY,

    title varchar(255) NOT NULL,
    notes text,
    start timestamp with time zone,
    has_time bool NOT NULL DEFAULT false,
    deadline date,

    position_key text NOT NULL,
    completed_at timestamp with time zone,
    deleted_at timestamp with time zone,

    updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    created_by uuid NOT NULL,

    FOREIGN KEY (created_by) REFERENCES app.users (id)
);

-- Create index for owner ids
CREATE INDEX idx_tasks_owner
ON app.tasks (created_by)
WHERE deleted_at IS NULL;

-- Create index for deleted tasks
CREATE INDEX idx_deleted_tasks
ON app.tasks (id)
WHERE deleted_at IS NOT NULL;

-- Create index for position key
CREATE INDEX idx_tasks_position
ON app.tasks(position_key);

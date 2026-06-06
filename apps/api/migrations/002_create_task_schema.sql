-- Task Schema
CREATE TABLE app.tasks (
    id uuid PRIMARY KEY,

    title varchar(255) NOT NULL,
    notes text,
    start_dt timestamp with time zone,
    has_time bool NOT NULL DEFAULT false,
    deadline date,

    completed_at timestamp with time zone,
    deleted_at timestamp with time zone,

    created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,

    created_by uuid NOT NULL,

    FOREIGN KEY (created_by) REFERENCES app.users (id)
);

-- Create index for Task owner ids
CREATE INDEX idx_tasks_owner
ON app.tasks (created_by)
WHERE deleted_at IS NULL;

-- Create indexes for deleted Tasks
CREATE INDEX ON app.tasks (id) WHERE deleted_at IS NOT NULL;

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON
app.tasks
TO cl_rw;

GRANT SELECT ON
app.tasks
TO cl_ro;

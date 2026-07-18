-- User Schema
CREATE TABLE app.users (
    id uuid PRIMARY KEY,

    display_name text NOT NULL,

    completed_task_retention interval,

    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

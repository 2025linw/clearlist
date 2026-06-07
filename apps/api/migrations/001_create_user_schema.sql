-- User Schema
CREATE TABLE app.users (
    id uuid PRIMARY KEY,

    display_name text NOT NULL,

    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON
app.users
TO cl_rw;

GRANT SELECT ON
app.users
TO cl_ro;

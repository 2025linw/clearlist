-- User Schema
CREATE TABLE app.users (
    id uuid PRIMARY KEY,
    auth_user_id uuid UNIQUE NOT NULL,

    display_name text NOT NULL,

    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

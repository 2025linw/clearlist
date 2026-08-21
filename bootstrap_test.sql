-- THIS IS ONLY TESTING BOOTSTRAP, DO NOT USE IN PRODUCTION
CREATE SCHEMA app IF NOT EXISTS;

-- Migration user
CREATE ROLE cl_migrate WITH LOGIN PASSWORD 'cl_migrate' NOSUPERUSER NOCREATEDB NOCREATEROLE;
ALTER ROLE cl_migrate SET search_path TO public;

GRANT CONNECT ON DATABASE testdb TO cl_migrate;
GRANT USAGE, CREATE ON SCHEMA public TO cl_migrate;

GRANT USAGE, CREATE ON SCHEMA app TO cl_migrate;
GRANT USAGE, CREATE ON SCHEMA auth TO cl_migrate;


-- API roles
CREATE ROLE cl_rw WITH NOSUPERUSER NOCREATEDB NOCREATEROLE;
CREATE ROLE cl_ro WITH NOSUPERUSER NOCREATEDB NOCREATEROLE;

GRANT USAGE ON SCHEMA app TO cl_rw;
GRANT USAGE ON SCHEMA app TO cl_ro;


-- Create database users
CREATE ROLE cl_api WITH LOGIN PASSWORD 'cl_api';
GRANT cl_rw TO cl_api;
ALTER ROLE cl_api SET search_path TO app;

CREATE ROLE cl_auth WITH LOGIN PASSWORD 'cl_auth';
ALTER ROLE cl_auth SET search_path TO auth;


-- Grant permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON
-- Users table
app.users,
-- Tasks table
app.tasks,
-- Tags table
app.tags,
app.task_tags
TO cl_rw;

GRANT SELECT ON
-- Users table
app.users,
-- Tasks table
app.tasks,
-- Tags table
app.tags,
app.task_tags
TO cl_ro;

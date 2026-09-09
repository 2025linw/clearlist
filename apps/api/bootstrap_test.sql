-- THIS IS ONLY TESTING BOOTSTRAP, DO NOT USE IN PRODUCTION


-- Create schemas
CREATE SCHEMA IF NOT EXISTS app;


-- Migration user (change password from 'cl_migrate' in prod)
CREATE ROLE cl_migrate WITH LOGIN PASSWORD 'cl_migrate' NOSUPERUSER NOCREATEDB NOCREATEROLE;
ALTER ROLE cl_migrate SET search_path TO public;

GRANT CONNECT ON DATABASE 'testdb' TO cl_migrate; -- update 'testdb' in prod

GRANT ALL ON SCHEMA public TO cl_migrate;
GRANT ALL ON SCHEMA app TO cl_migrate;


-- Create user (change password from 'cl_api')
CREATE ROLE cl_api WITH LOGIN PASSWORD 'cl_api';

GRANT USAGE ON SCHEMA app TO cl_api;
ALTER ROLE cl_api SET search_path TO app;

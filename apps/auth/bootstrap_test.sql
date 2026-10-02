-- THIS IS ONLY TESTING BOOTSTRAP, DO NOT USE IN PRODUCTION


-- Create schemas
CREATE SCHEMA IF NOT EXISTS auth;


-- Migration user (change password from 'cl_migrate_auth' in prod)
CREATE ROLE cl_migrate_auth WITH LOGIN PASSWORD 'cl_migrate_auth' NOSUPERUSER NOCREATEDB NOCREATEROLE;
ALTER ROLE cl_migrate_auth SET search_path TO auth;

GRANT CONNECT ON DATABASE "testdb" TO cl_migrate_auth; -- update 'testdb' in prod

GRANT ALL ON SCHEMA auth TO cl_migrate_auth;


-- Create user (change password from 'cl_auth')
CREATE ROLE cl_auth WITH LOGIN PASSWORD 'cl_auth';

GRANT USAGE ON SCHEMA auth TO cl_auth;
ALTER ROLE cl_auth SET search_path TO auth;

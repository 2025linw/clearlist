-- Tag Schema
CREATE TABLE app.tags (
    id uuid PRIMARY KEY,

    label varchar(255) NOT NULL,
    category varchar(255),

    position_key text NOT NULL,

    updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    created_by uuid NOT NULL,

    FOREIGN KEY (created_by) REFERENCES app.users (id)
);

-- Create index for owner ids
CREATE INDEX idx_tags_owner
ON app.tags (created_by);

-- Create index for position key
CREATE INDEX idx_tags_position
ON app.tags(position_key);

-- Task-Tag Table
CREATE TABLE app.task_tags (
    task_id uuid NOT NULL,
    tag_id uuid NOT NULL,

    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY (task_id) REFERENCES app.tasks (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES app.tags (id) ON DELETE CASCADE
);

-- Trigger to ensure that no task or tag is 'deleted' when adding task-tags
CREATE OR REPLACE FUNCTION app.check_task_not_deleted()
RETURNS trigger AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM app.tasks
        WHERE id = NEW.task_id
        AND deleted_at IS NOT NULL
    ) THEN
        RAISE EXCEPTION USING
            MESSAGE = 'resource_deleted',
            DETAIL = json_build_object(
                'resource_type', 'task',
                'resource_id', NEW.task_id
            )::text;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trig_check_task_not_deleted
BEFORE INSERT ON app.task_tags
FOR EACH ROW
EXECUTE FUNCTION app.check_task_not_deleted();

-- Trigger to ensure that task and tag are owned by the same user
CREATE OR REPLACE FUNCTION app.check_task_tag_owner()
RETURNS trigger AS $$
DECLARE
    task_owner uuid;
    tag_owner uuid;
BEGIN
    SELECT created_by INTO task_owner
    FROM app.tasks
    WHERE id = NEW.task_id;

    IF task_owner IS NULL THEN
        RAISE EXCEPTION USING
            MESSAGE = 'resource_not_found',
            DETAIL = json_build_object(
                'resource_type', 'task',
                'resource_id', NEW.task_id
            )::text;
    END IF;

    SELECT created_by INTO tag_owner
    FROM app.tags
    WHERE id = NEW.tag_id;

    IF tag_owner IS NULL THEN
        RAISE EXCEPTION USING
            MESSAGE = 'resource_not_found',
            DETAIL = json_build_object(
                'resource_type', 'tag',
                'resource_id', NEW.tag_id
            )::text;
    END IF;

    IF task_owner <> tag_owner THEN
        RAISE EXCEPTION USING
            MESSAGE = 'ownership_mismatch',
            DETAIL = json_build_object(
                'source_type', 'tag',
                'source_id', NEW.tag_id,
                'target_type', 'task',
                'target_id', NEW.task_id
            )::text;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trig_check_task_tag_owner
BEFORE INSERT ON app.task_tags
FOR EACH ROW
EXECUTE FUNCTION app.check_task_tag_owner();

-- Tag Category Schema
CREATE TABLE app.categories (
    id uuid PRIMARY KEY,

    category_name varchar(255),
    position_key text NOT NULL,

    created_by uuid NOT NULL,

    FOREIGN KEY (created_by) REFERENCES app.users (id),

    UNIQUE (category_name, created_by)
);

-- Tag Schema
CREATE TABLE app.tags (
    id uuid PRIMARY KEY,

    label varchar(255) NOT NULL,
    category_id uuid,

    position_key text NOT NULL,

    updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    created_by uuid NOT NULL,

    FOREIGN KEY (category_id) REFERENCES app.categories (id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES app.users (id),

    UNIQUE NULLS NOT DISTINCT (label, category_id, created_by)
);

-- Create index for owner ids
CREATE INDEX idx_tags_owner
ON app.tags (created_by);

-- Create index for position key
CREATE INDEX idx_tags_position
ON app.tags(position_key);

-- Trigger to ensure that tag and tag category have same owner
CREATE OR REPLACE FUNCTION app.check_tag_category_owner()
RETURNS trigger AS $$
DECLARE
    category_owner uuid;
BEGIN
    IF NEW.category_id IS NULL THEN
        RETURN NEW;
    END IF;

    SELECT created_by INTO category_owner
    FROM app.categories
    WHERE id = NEW.category_id;

    IF category_owner IS NULL THEN
        RAISE EXCEPTION USING
            MESSAGE = 'resource_not_found',
            DETAIL = json_build_object(
                'resource_type', 'category',
                'resource_id', NEW.category_id
            )::text;
    END IF;

    IF NEW.created_by <> category_owner THEN
        RAISE EXCEPTION USING
            MESSAGE = 'ownership_mismatch',
            DETAIL = json_build_object(
                'source_type', 'category',
                'source_id', NEW.category_id,
                'target_type', 'tag',
                'target_id', NEW.id
            )::text;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trig_check_tag_category_owner
BEFORE INSERT OR UPDATE ON app.tags
FOR EACH ROW
EXECUTE FUNCTION app.check_tag_category_owner();

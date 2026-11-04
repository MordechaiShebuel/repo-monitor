BEGIN;

CREATE TABLE application (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT application_name_not_blank CHECK (length(trim(name)) > 0)
);

CREATE TABLE endpoint (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    app_id      BIGINT NOT NULL REFERENCES application(id) ON DELETE CASCADE,
    url         TEXT NOT NULL,
    CONSTRAINT endpoint_url_not_blank CHECK (length(trim(url)) > 0),
    CONSTRAINT endpoint_app_url_unique UNIQUE (app_id, url)
);

CREATE TABLE app_version (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    app_id      BIGINT NOT NULL UNIQUE
                REFERENCES application(id) ON DELETE CASCADE,
    version     TEXT NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT app_version_not_blank CHECK (length(trim(version)) > 0)
);

COMMIT;

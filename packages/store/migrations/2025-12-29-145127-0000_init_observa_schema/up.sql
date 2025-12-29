-- Your SQL goes here


-- ENUM TYPES
CREATE TYPE monitor_status AS ENUM ('Up', 'Down', 'Unknown');

CREATE TYPE alert_type AS ENUM ('DownAlert', 'UpAlert', 'ResponseTimeAlert');

CREATE TYPE plan_type AS ENUM ('Free', 'Pro');

CREATE TYPE subscription_status AS ENUM ('Active', 'Cancelled', 'Expired');

CREATE TYPE channel_type AS ENUM ('Email', 'Sms', 'VoiceCall', 'Webhook');


-- TABLES
CREATE TABLE "user" (
    id TEXT PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE subscription (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL UNIQUE,
    plan plan_type NOT NULL,
    status subscription_status NOT NULL,
    starts_at TIMESTAMP(3) NOT NULL,
    ends_at TIMESTAMP(3),

    CONSTRAINT subscription_user_id_fkey
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE
);

CREATE TABLE notification_channel (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    type channel_type NOT NULL,
    value TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT notification_channel_user_id_fkey
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE
);

CREATE TABLE monitor (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    name TEXT,
    interval INTEGER NOT NULL DEFAULT 300,
    timeout_ms INTEGER NOT NULL DEFAULT 5000,
    is_paused BOOLEAN NOT NULL DEFAULT false,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT monitor_user_id_fkey
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE
);

CREATE TABLE region (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE checks (
    id TEXT PRIMARY KEY,
    response_time_ms INTEGER NOT NULL,
    status monitor_status NOT NULL,
    region_id TEXT NOT NULL,
    monitor_id TEXT NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT checks_region_id_fkey
        FOREIGN KEY (region_id)
        REFERENCES region(id)
        ON DELETE RESTRICT,

    CONSTRAINT checks_monitor_id_fkey
        FOREIGN KEY (monitor_id)
        REFERENCES monitor(id)
        ON DELETE CASCADE
);

CREATE TABLE alert (
    id TEXT PRIMARY KEY,
    type alert_type NOT NULL,
    message TEXT NOT NULL,
    sent_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    monitor_id TEXT NOT NULL,

    CONSTRAINT alert_monitor_id_fkey
        FOREIGN KEY (monitor_id)
        REFERENCES monitor(id)
        ON DELETE CASCADE
);

CREATE TABLE status_page (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    user_id TEXT NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT status_page_user_id_fkey
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE
);


-- INDEXES
CREATE INDEX monitor_user_id_idx
    ON monitor(user_id);

CREATE INDEX checks_monitor_id_created_at_idx
    ON checks(monitor_id, created_at);

CREATE INDEX checks_region_id_idx
    ON checks(region_id);

CREATE INDEX alert_monitor_id_sent_at_idx
    ON alert(monitor_id, sent_at);
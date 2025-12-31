-- 1. DROP FKs
ALTER TABLE subscription DROP CONSTRAINT subscription_user_id_fkey;
ALTER TABLE notification_channel DROP CONSTRAINT notification_channel_user_id_fkey;
ALTER TABLE monitor DROP CONSTRAINT monitor_user_id_fkey;
ALTER TABLE checks DROP CONSTRAINT checks_region_id_fkey;
ALTER TABLE checks DROP CONSTRAINT checks_monitor_id_fkey;
ALTER TABLE alert DROP CONSTRAINT alert_monitor_id_fkey;
ALTER TABLE status_page DROP CONSTRAINT status_page_user_id_fkey;

-- 2. REVERT ID TYPES
ALTER TABLE status_page
    ALTER COLUMN user_id TYPE TEXT USING user_id::text,
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE alert
    ALTER COLUMN monitor_id TYPE TEXT USING monitor_id::text,
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE checks
    ALTER COLUMN monitor_id TYPE TEXT USING monitor_id::text,
    ALTER COLUMN region_id TYPE TEXT USING region_id::text,
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE region
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE monitor
    ALTER COLUMN user_id TYPE TEXT USING user_id::text,
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE notification_channel
    ALTER COLUMN user_id TYPE TEXT USING user_id::text,
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE subscription
    ALTER COLUMN user_id TYPE TEXT USING user_id::text,
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE "user"
    ALTER COLUMN id DROP DEFAULT,
    ALTER COLUMN id TYPE TEXT USING id::text;

-- 3. RECREATE FKs
ALTER TABLE subscription
    ADD CONSTRAINT subscription_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;

ALTER TABLE notification_channel
    ADD CONSTRAINT notification_channel_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;

ALTER TABLE monitor
    ADD CONSTRAINT monitor_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;

ALTER TABLE checks
    ADD CONSTRAINT checks_region_id_fkey
    FOREIGN KEY (region_id) REFERENCES region(id) ON DELETE RESTRICT;

ALTER TABLE checks
    ADD CONSTRAINT checks_monitor_id_fkey
    FOREIGN KEY (monitor_id) REFERENCES monitor(id) ON DELETE CASCADE;

ALTER TABLE alert
    ADD CONSTRAINT alert_monitor_id_fkey
    FOREIGN KEY (monitor_id) REFERENCES monitor(id) ON DELETE CASCADE;

ALTER TABLE status_page
    ADD CONSTRAINT status_page_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;

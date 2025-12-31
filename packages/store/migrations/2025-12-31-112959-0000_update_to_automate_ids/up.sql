CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 1. DROP FKs
ALTER TABLE subscription DROP CONSTRAINT subscription_user_id_fkey;
ALTER TABLE notification_channel DROP CONSTRAINT notification_channel_user_id_fkey;
ALTER TABLE monitor DROP CONSTRAINT monitor_user_id_fkey;
ALTER TABLE checks DROP CONSTRAINT checks_region_id_fkey;
ALTER TABLE checks DROP CONSTRAINT checks_monitor_id_fkey;
ALTER TABLE alert DROP CONSTRAINT alert_monitor_id_fkey;
ALTER TABLE status_page DROP CONSTRAINT status_page_user_id_fkey;

-- 2. ALTER ID TYPES
ALTER TABLE "user"
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE subscription
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN user_id TYPE UUID USING user_id::uuid;

ALTER TABLE notification_channel
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN user_id TYPE UUID USING user_id::uuid;

ALTER TABLE monitor
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN user_id TYPE UUID USING user_id::uuid;

ALTER TABLE region
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE checks
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN region_id TYPE UUID USING region_id::uuid,
    ALTER COLUMN monitor_id TYPE UUID USING monitor_id::uuid;

ALTER TABLE alert
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN monitor_id TYPE UUID USING monitor_id::uuid;

ALTER TABLE status_page
    ALTER COLUMN id TYPE UUID USING id::uuid,
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN user_id TYPE UUID USING user_id::uuid;

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

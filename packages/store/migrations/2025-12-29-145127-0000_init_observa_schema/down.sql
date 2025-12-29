-- This file should undo anything in `up.sql`

-- DROP INDEXES
DROP INDEX IF EXISTS alert_monitor_id_sent_at_idx;
DROP INDEX IF EXISTS checks_region_id_idx;
DROP INDEX IF EXISTS checks_monitor_id_created_at_idx;
DROP INDEX IF EXISTS monitor_user_id_idx;

-- DROP TABLES
DROP TABLE IF EXISTS status_page;
DROP TABLE IF EXISTS alert;
DROP TABLE IF EXISTS checks;
DROP TABLE IF EXISTS region;
DROP TABLE IF EXISTS monitor;
DROP TABLE IF EXISTS notification_channel;
DROP TABLE IF EXISTS subscription;
DROP TABLE IF EXISTS "user";

-- DROP ENUM TYPES
DROP TYPE IF EXISTS channel_type;
DROP TYPE IF EXISTS subscription_status;
DROP TYPE IF EXISTS plan_type;
DROP TYPE IF EXISTS alert_type;
DROP TYPE IF EXISTS monitor_status;
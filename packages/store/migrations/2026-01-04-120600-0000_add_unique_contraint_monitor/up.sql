ALTER TABLE monitor
ADD CONSTRAINT monitor_user_url_unique UNIQUE (user_id, url);

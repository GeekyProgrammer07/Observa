ALTER TABLE notification_channel
ADD CONSTRAINT user_notification_value_unique UNIQUE (user_id, value);

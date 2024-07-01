CREATE DATABASE IF NOT EXISTS stats;

DROP TABLE user_stats;

CREATE TABLE user_stats (
  email String,
  name String,
  gender Enum8('unknown' = 0, 'male' = 1, 'female' = 2) DEFAULT 'unknown',
  created_at DateTime DEFAULT now(),
  last_visited_at DateTime,
  last_watched_at DateTime,
  recent_watched Array(Int32),
  viewed_but_not_started Array(Int32),
  started_but_not_finished Array(Int32),
  finished Array(Int32),
  last_email_notification DateTime,
  last_in_app_notification DateTime,
  last_sms_notification DateTime
) ENGINE = MergeTree()
PRIMARY KEY email
ORDER BY (email, created_at, last_visited_at, last_watched_at, last_email_notification, last_in_app_notification, last_sms_notification);

-- DEV ONLY - Brute Force DROP DB (for loval dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'post_dev' OR datname  = 'post_article';
DROP DATABASE IF EXISTS post_article;
DROP USER IF EXISTS post_dev;

-- DEV ONLY - Dev only passsword (for local dev and unit test).
CREATE USER post_dev PASSWORD 'dev_only_pwd';
CREATE DATABASE post_article owner post_dev ENCODING = 'UTF-8';

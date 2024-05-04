-- DEV ONLY - Brute Force DROP DB (for loval dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'shelter_dev' OR datname  = 'animals_shelter';
DROP DATABASE IF EXISTS animals_shelter;
DROP USER IF EXISTS shelter_dev;

-- DEV ONLY - Dev only passsword (for local dev and unit test).
CREATE USER shelter_dev PASSWORD 'dev_only_pwd';
CREATE DATABASE animals_shelter owner shelter_dev ENCODING = 'UTF-8';

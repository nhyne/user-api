-- This file should undo anything in `up.sql`
DROP INDEX email_idx;

DROP TABLE users;

DROP EXTENSION IF EXISTS "uuid-ossp";

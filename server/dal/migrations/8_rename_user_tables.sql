ALTER TYPE user_type RENAME VALUE 'Koala' TO 'Standard';
ALTER TYPE user_type RENAME VALUE 'Service' TO 'Token';

ALTER TABLE users RENAME TO standard_users;
ALTER TABLE user_sessions RENAME TO standard_user_sessions;
ALTER TABLE service_token_user RENAME TO token_users;
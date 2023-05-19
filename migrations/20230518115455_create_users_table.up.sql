-- Create the 'users' table.
CREATE TABLE `users` (
	`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
	`account_created_at` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	`password_reset_at` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	`profile_picture_url` VARCHAR(8192) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`username` VARCHAR(128) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`password` VARCHAR(1024) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`email` VARCHAR(1024) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`email_is_verified` BIT(1) NOT NULL DEFAULT b'0',
	`password_reset_is_required` BIT(1) NOT NULL DEFAULT b'0',
	`account_is_locked` BIT(1) NOT NULL DEFAULT b'0',
	`account_is_banned` BIT(1) NOT NULL DEFAULT b'0',
	PRIMARY KEY (`id`) USING BTREE,
	UNIQUE INDEX `username` (`username`) USING BTREE,
	UNIQUE INDEX `email` (`email`) USING HASH
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB
AUTO_INCREMENT=1;

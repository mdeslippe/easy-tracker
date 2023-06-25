-- Create the 'files' table.
CREATE TABLE `files` (
	`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
	`user_id` BIGINT(20) UNSIGNED NOT NULL,
	`file_created_at` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	`mime_type` VARCHAR(1024) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`name` VARCHAR(1024) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`data` LONGBLOB NOT NULL,
	PRIMARY KEY (`id`) USING BTREE,
	INDEX `FK__users` (`user_id`) USING BTREE,
	CONSTRAINT `FK__users` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON UPDATE NO ACTION ON DELETE CASCADE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB
ROW_FORMAT=COMPRESSED
AUTO_INCREMENT=1;

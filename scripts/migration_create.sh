MIGRATION_NAME=$1
DB_FOLDER=diesel

diesel migration generate $MIGRATION_NAME --migration-dir $DB_FOLDER/migrations

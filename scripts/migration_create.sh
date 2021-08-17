MIGRATION_NAME=$1
DB_FOLDER=crates/base/db

diesel migration generate $MIGRATION_NAME --migration-dir $DB_FOLDER/migrations

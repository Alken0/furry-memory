DB_FOLDER=crates/base/db

diesel migration run --migration-dir $DB_FOLDER/migrations --database-url $DB_FOLDER/db.sqlite
diesel migration redo --migration-dir $DB_FOLDER/migrations	--database-url $DB_FOLDER/db.sqlite # verify

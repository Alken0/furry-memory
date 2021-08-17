#[rocket_sync_db_pools::database("diesel")]
pub struct Database(diesel::SqliteConnection);

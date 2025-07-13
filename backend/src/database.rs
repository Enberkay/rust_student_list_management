use sea_orm::{Database, DatabaseConnection, EntityTrait, Schema, ConnectionTrait};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let db = Database::connect(&database_url).await?;
    
    // Create tables if they don't exist
    create_tables(&db).await?;
    
    Ok(db)
}

async fn create_tables(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Schema::new(db.get_database_backend());
    
    // Create users table if it doesn't exist
    let create_users_table = schema.create_table_from_entity(crate::models::Entity);
    let sql = db.get_database_backend().build(&create_users_table);
    
    // Try to create table, ignore if already exists
    match db.execute(sql).await {
        Ok(_) => println!("Users table created successfully"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                println!("Users table already exists, skipping creation");
            } else {
                return Err(e.into());
            }
        }
    }
    
    Ok(())
} 
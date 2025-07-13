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
    
    // Create users table
    let create_users_table = schema.create_table_from_entity(crate::models::Entity);
    db.execute(db.get_database_backend().build(&create_users_table)).await?;
    
    Ok(())
} 
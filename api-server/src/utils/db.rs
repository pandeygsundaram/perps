use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};

pub async fn connect_db() -> PgPool {

    PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &std::env::var("DATABASE_URL")
                .unwrap()
        )
        .await
        .unwrap()
}
use sqlx::{Connection, Row};

pub async fn index() -> String {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("Missing env DATABASE_URL");
    // let url = "postgresql://kawa:kawa@localhost:5432/kawa";
    // let url = "postgresql://kawa:kawa@database:5432/kawa";
    let mut conn = sqlx::postgres::PgConnection::connect(&url).await.unwrap();

    let query = "select * from users where id=1";

    // let res = sqlx::query("SELECT 1 + 1 as sum")
    let res = sqlx::query(query).fetch_one(&mut conn).await.unwrap();

    // let sum: i32 = res.get("sum");
    let username: String = res.get("username");

    format!("{}", username)
}

use sqlx::{Connection, Row};

pub async fn index() -> String {
    // let url = "postgresql://kawa:kawa@localhost:5432/kawa";
    let url = "postgresql://kawa:kawa@database:5432/kawa";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await.unwrap();

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&mut conn)
        .await
        .unwrap();

    let sum: i32 = res.get("sum");

    format!("{}", sum)
}

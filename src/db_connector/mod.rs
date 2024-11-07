use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn init_db_connection() -> anyhow::Result<Surreal<Client>> {
    let conn = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    conn.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    conn.use_ns("copycat_code")
        .use_db("copycat_code_db")
        .await?;
    Ok(conn)
}

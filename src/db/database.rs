use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};



#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "toor",
        })
        .await?;
        client.use_ns("surreal").use_db("pizzas").await.unwrap();
        Ok(Database {
            client,
            namespace: String::from("surreal"),
            db_name: String::from("pizzas"),
        })
    }
}
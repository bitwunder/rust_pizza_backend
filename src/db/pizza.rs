use actix_web::web::Data;
use surrealdb::Error;
use crate::db::Database;
use async_trait::async_trait;

use crate::models::Pizza;



#[async_trait]
pub trait PizzaDataTrait {
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>>;
    async fn add_pizza(db: &Data<Database>, pizza: Pizza) -> Option<Pizza>;
    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza>;
}

#[async_trait]
impl PizzaDataTrait for Database {
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>> {
        if let Ok(list) = db.client.select("pizza").await {
            return Some(list);
        }
        None
    }

    async fn add_pizza(db: &Data<Database>, pizza: Pizza) -> Option<Pizza> {
        db.client
            .create(("pizza", pizza.uuid.clone()))
            .content(pizza)
            .await
            .ok()?
    }

    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza> {
        let found_pizza: Result<Option<Pizza>, Error> = db
            .client
            .select(("pizza", &uuid)).await;
        if let Ok(Some(_found_pizza)) = found_pizza {
            let updated_pizza = db.client.update(("pizza", &uuid))
                .merge(Pizza {
                    uuid,
                    name: String::from("sold"),
                })
                .await;
            if let Ok(Some(pizza)) = updated_pizza {
                return Some(pizza);
            }
        }
        None
    }
}
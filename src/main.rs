use actix_web::{
    get, post, patch, HttpServer, App,
    web::{Json, Path, Data},
};
use validator::Validate;
use uuid::Uuid;

use rust_backend::{db::{Database, PizzaDataTrait}, models::*, errors::*};


#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    match Database::get_all_pizzas(&db).await {
        Some(pizzas) => Ok(Json(pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

#[post("/buypizza")]
async fn buy_pizza(
    db: Data<Database>,
    body: Json<BuyPizzaRequest>
) -> Result<Json<Pizza>, PizzaError>
{
    match body.validate() {
        Ok(_) => {
            let name = body.pizza_name.clone();
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4()
                .simple()
                .encode_lower(&mut buffer)
                .to_owned();

            match Database::add_pizza(&db, Pizza::new(new_uuid, name)).await {
                Some(pizza) => Ok(Json(pizza)),
                None => Err(PizzaError::CreationError)
            }
        }
        Err(_) => Err(PizzaError::CreationError)
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(
    db: Data<Database>,
    update_pizza_url: Path<UpdatePizzaURL>
) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let updated_pizza = Database::update_pizza(&db, uuid).await;

    match updated_pizza {
        Some(pizza) => Ok(Json(pizza)),
        None => Err(PizzaError::NoSuchPizza),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await
        .expect("Cannot initialize database connection");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

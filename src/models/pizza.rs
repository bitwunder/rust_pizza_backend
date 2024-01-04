use serde::{Deserialize, Serialize};
use validator::Validate;



#[derive(Validate, Serialize, Deserialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "Pizza name is required"))]
    pub pizza_name: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}


#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub name: String,
}

impl Pizza {
    pub fn new(uuid: String, name: String) -> Self {
        Self { uuid, name, }
    }
}
use crate::models::user_model::Balance;

pub struct OnRampRequestReturnType {
    message: String,
    data : Option<Balance>,
}

pub struct OnRampRequestPayloadBodyType {
    pub amount : i32
}

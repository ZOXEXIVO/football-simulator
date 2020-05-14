use serde::{Serialize};

#[derive(Serialize)]
pub struct PlayerDto<'p> {
    pub first_name: &'p str,
    pub last_name: &'p str,
    pub middle_name: &'p str,
}

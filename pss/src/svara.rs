use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SvaraEnum {
    Svara1, Svara2, Svara3, Svara4, Svara5,
    Svara6, Svara7, Svara8, Svara9, Svara10,
    Svara11, Svara12, Svara13, Svara14, Svara15,
    Svara16, Svara17, Svara18, Svara19, Svara20, Svara21,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Svara {
    pub svara: SvaraEnum,
}

impl Svara {
    pub fn new(svara: SvaraEnum) -> Self {
        Svara { svara }
    }
}

use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]  // what is Debug ?
pub struct Pokemon {
    pub id: i32,
    pub name: Names,
}

//#[serde(rename_all = "PascalCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Names {                                       // pub neccessary here ?
    pub english: String,
    pub french: String,
}

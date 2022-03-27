use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub result: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BuildBoard {
    TakeBoard1 {board1: [i32;100]},
    TakeBoard2 {identity: String, board2: [i32;100] },
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GeneralAttack {
    // GetCount returns the current count as a json-encoded number
    GetResultBoard1 {},
    GetResultBoard2{identity: String},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AttackResult {
    pub result: i32,
}

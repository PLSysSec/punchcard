use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrMany<O, M> {
    One(O),
    Many(Vec<M>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrDictionary<O, D> {
    One(O),
    Dictionary(D),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BoolOrExpr {
    Bool(bool),
    Expr(String),    
}

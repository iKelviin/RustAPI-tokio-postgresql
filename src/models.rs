use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Produto{
    #[serde(skip_deserializing)]
    pub id: i64,
    #[serde(skip_deserializing)]
    pub uuid: Uuid,
    pub nome: String,
    pub ean: String,
    pub descricao: Option<String>,
    pub volume: String,
    pub departamento: String,
    pub categoria: String,
    pub subcategoria: String,
    pub imagemgrande: String,
    pub imagempequena: String
}

#[derive(Deserialize)]
pub struct NewProduto{
    pub nome: String,
    pub ean: String,
    pub descricao: Option<String>,
    pub volume: String,
    pub departamento: String,
    pub categoria: String,
    pub subcategoria: String,
    pub imagemgrande: String,
    pub imagempequena: String
}
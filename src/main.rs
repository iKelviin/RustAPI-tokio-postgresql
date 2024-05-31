#[macro_use] extern crate rocket;

mod models;
mod lib;


use tokio_postgres::{NoTls, Error};
use rocket::serde::json::Json;
use rocket::{response::status, serde::json::{json, Value}};
use models::{Produto, NewProduto};
use lib::MyUuid;


async fn get_db_connection() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=Localizei", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

#[get("/produtos")]
async fn get_produtos() -> Value {
    let client = get_db_connection().await.expect("Failed to connect to DB");
    let rows = client.query("select * from tbl_produtos order by id asc limit 20000", &[]).await.expect("DB Error");
    let produtos: Vec<Produto> = rows.iter().map(|row| {
        Produto {
            id: row.get("id"),
            uuid: row.get("uuid"),
            nome: row.get("nome"),
            ean: row.get("ean"),
            descricao: row.get("descricao"),
            volume: row.get("volume"),
            departamento: row.get("departamento"),
            categoria: row.get("categoria"),
            subcategoria: row.get("subCategoria"),
            imagemgrande: row.get("imagemGrande"),
            imagempequena: row.get("imagemPequena"),
        }
    }).collect();
    json!(produtos)
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_produtos
        ]).register("/", catchers![
            not_found
        ])
        .launch().await;
}
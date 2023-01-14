use todo_api;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let server = todo_api::rocket().launch().await?;
    println!("Ricket: {:?}", server);

    Ok(())
}

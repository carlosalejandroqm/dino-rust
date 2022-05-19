use tide::Response;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dino {
    id: Uuid,
    name: String,
    weight: i32,
    diet: String,
    user_id: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/dinos").post(|mut req: Request<()>| async move {
        let body = req.body_string().await?;
        println!("{:?}", body);
        let mut res = Response::new(201);
        res.set_body(String::from("created!"));
        Ok(res)
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
   

}
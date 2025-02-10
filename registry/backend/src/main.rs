use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use dotenvy::dotenv;
use rocket::{fairing::{Fairing, Info, Kind}, http::{Header, Method, Status}, response::content::RawHtml, State};

mod schema;
mod oci;

// MARK: Start Rocket Definition
#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Cors",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        if req.method() == Method::Options {
            res.set_status(Status::NoContent);
            res.set_header(Header::new("Access-Control-Allow-Methods", "POST, OPTIONS"));
        }
        
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/graphql", data = "<req>", format = "application/json")]
async fn graphql_request(schema: &State<schema::TelcharSchema>, req: GraphQLRequest) -> GraphQLResponse {
    // req.execute(schema.deref()).await
    req.execute(schema.inner()).await
}

#[get("/graphql")]
async fn graphql() -> RawHtml<String> {
    rocket::response::content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[launch]
fn rocket() -> _ {
    let _ = dotenv();
    
    let schema = schema::build_schema();

    rocket::build()
        .manage(schema)
        .mount("/", routes![index, graphql, graphql_request])
        .attach(CORS)
}

// MARK: End Rocket Definition
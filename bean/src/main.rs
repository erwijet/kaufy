mod db;
mod graphql;
mod jwt;
mod middleware;

use std::error::Error;

use async_graphql::http::GraphiQLSource;
use entity::sea_orm::{ActiveModelTrait, Set};
use middleware::JwtMiddleware;
use poem::{
    error::{InternalServerError, NotFoundError, Unauthorized},
    http::{Request, StatusCode, Uri},
    listener::TcpListener,
    middleware::Cors,
    web::{Html, Path, Query},
    *,
};

use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
#[cfg(debug_assertions)]
use dotenv::dotenv;
use graphql::schema::build_schema;
use poem::{get, handler, IntoResponse, Route};
use serde::{Deserialize, Serialize};

use crate::{graphql::schema::OwnerID, jwt::Claims};

#[derive(Deserialize)]
struct GoogleTokenInfo {
    email: String,
    given_name: String,
    family_name: String,
    hd: String,
    picture: String,
}

#[derive(Deserialize)]
struct AuthReq {
    token: String,
}

#[derive(Serialize)]
struct AuthRes {
    token: String,
}

#[poem::handler]
/// Allows the client to exchange a google `access_token` for a signed jwt
async fn auth_with_google(Query(AuthReq { token }): Query<AuthReq>) -> poem::Result<String> {
    let db = db::Database::new().await;

    let res_txt = reqwest::get(format!(
        "https://www.googleapis.com/oauth2/v1/userinfo?alt=json&access_token={token}"
    ))
    .await
    .map_err(Unauthorized)?
    .text()
    .await
    .map_err(InternalServerError)?;

    let GoogleTokenInfo {
        email,
        given_name,
        family_name,
        picture,
        hd,
        ..
    } = serde_json::from_str(&res_txt).map_err(InternalServerError)?;

    if hd != "bryx.com" {
        return Err(poem::Error::from_string(
            format!("Tokens are only permitted to be administered to 'bryx' accounts"),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let lookup_res = entity::user::Entity::find_by_email(&email)
        .one(&db)
        .await
        .map_err(InternalServerError)?;

    let user = if let Some(found) = lookup_res {
        found
    } else {
        entity::user::ActiveModel {
            email: Set(email),
            family_name: Set(family_name),
            given_name: Set(given_name),
            picture: Set(picture),
            ..Default::default()
        }
        .insert(&db)
        .await
        .map_err(InternalServerError)?
    };

    let token = jwt::create_jwt(user.into())?;

    Ok(serde_json::to_string_pretty(&AuthRes { token }).unwrap())
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[handler]
async fn index(req: GraphQLRequest, data: poem::web::Data<&Claims>) -> GraphQLResponse {
    let mut req = req.0;

    let schema = build_schema(data.id as OwnerID).await;
    return schema.execute(req).await.into();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let app = Route::new()
        .at("/graphql", get(graphiql).post(index).with(JwtMiddleware))
        .at("/oauth/google", post(auth_with_google))
        .with(Cors::new());

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;

    Ok(())
}

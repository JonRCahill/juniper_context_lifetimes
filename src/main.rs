#[macro_use]
extern crate rocket;

use juniper::{EmptyMutation, EmptySubscription};
use rocket::State;

use juniper_context_lifetimes::{Context, QueryRoot};

pub type Schema<'a> = juniper::RootNode<
    'static,
    QueryRoot<'a>,
    EmptyMutation<Context<'a>>,
    EmptySubscription<Context<'a>>,
>;

#[get("/graphql?<request>")]
pub async fn get_graphql_handler<'a>(
    request: juniper_rocket_async::GraphQLRequest,
    schema: State<'_, Schema<'a>>,
    context: Context<'a>,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute(&schema, &context).await
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(Schema::new(
            QueryRoot {
                _marker: std::marker::PhantomData,
            },
            EmptyMutation::<Context>::new(),
            EmptySubscription::<Context>::new(),
        ))
        .mount("/", routes![get_graphql_handler])
}

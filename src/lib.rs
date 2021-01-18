use juniper::{graphql_object, FieldResult};
use rocket::request::{self, FromRequest, Request};

pub struct Context<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> juniper::Context for Context<'a> {}

impl<'a> Context<'a> {
    pub async fn get_something(&self) -> FieldResult<String> {
        Ok("Something".into())
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Context<'a> {
    type Error = ();

    async fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        rocket::outcome::Outcome::Success(Context {
            _marker: std::marker::PhantomData,
        })
    }
}

pub struct QueryRoot<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

#[graphql_object(Context = Context<'a>)]
impl<'a> QueryRoot<'a> {
    async fn get_something(context: &Context<'a>) -> FieldResult<String> {
        context.get_something().await
    }
}

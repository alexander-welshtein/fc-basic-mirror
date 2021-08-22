use juniper::{GraphQLObject, FieldResult};
use crate::schemas::root::{QueryRoot, Context};
use crate::db::connect;
use tokio_postgres::{Client, Row};

#[derive(GraphQLObject)]
#[graphql(description = "Implementation of basic flashcard entity")]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
    pub date: String
}

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[derive(description = "List of all cards")]
    async fn card(context: &Context) -> Vec<Row> {
        let client: Client = context.pool.get();

        let result = client.query("SELECT * FROM card", &[]).await?;

        result
    }
}
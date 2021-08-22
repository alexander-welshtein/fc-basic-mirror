use deadpool_postgres::Client;
use juniper::GraphQLObject;

use crate::schemas::root::{Context, QueryRoot};

#[derive(GraphQLObject)]
#[graphql(description = "Basic implementation of flashcard entity")]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
    pub date: String,
}

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[derive(description = "List of all cards")]
    async fn card(context: &Context) -> Vec<Card> {
        let client: Client = match context.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("DB :: error :: {}", e);
                return Vec::default();
            }
        };

        let rows = match client.query("SELECT * FROM card", &[]).await {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("DB :: error :: {}", e);
                return Vec::default();
            }
        };

        rows
            .into_iter()
            .map(|row| Card {
                id: row.get(0),
                front: row.get(1),
                back: row.get(2),
                date: row.get(3),
            })
            .collect()
    }
}
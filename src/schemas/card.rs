use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::schemas::root::{Query, SchemaContext};

pub struct Card {
    pub id: i64,
    pub front: String,
    pub back: String,
    pub date: DateTime<Utc>,
}

impl From<&PgRow> for Card {
    fn from(row: &PgRow) -> Self {
        Self {
            id: row.try_get("id").unwrap(),
            front: row.try_get("front").unwrap(),
            back: row.try_get("back").unwrap(),
            date: row.try_get("date").unwrap(),
        }
    }
}

#[Object]
impl Card {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn front(&self) -> String {
        self.front.to_string()
    }

    async fn back(&self) -> String {
        self.back.to_string()
    }

    async fn date(&self) -> String { self.date.to_string() }
}

#[Object]
impl Query {
    async fn cards<'ctx>(&self, context: &Context<'ctx>) -> Vec<Card> {
        let rows = match sqlx::query(include_str!("../../db/select_cards.sql"))
            .fetch_all(context.data_unchecked::<SchemaContext>().pool.as_ref())
            .await
        {
            Ok(v) => v,
            Err(_) => {
                eprintln!("[E] DB :: Select cards");
                Vec::default()
            }
        };

        rows.iter().map(Card::from).collect()
    }
}
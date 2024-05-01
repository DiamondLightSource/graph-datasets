/// Collection of graphql entities
mod entity;

use async_graphql::{
    types::connection::*, ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema,
    SchemaBuilder,
};
use entity::{DataCollection, Session};
use models::data_collection;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

/// The GraphQL schema exposed by the service
pub type RootSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// A schema builder for the service
pub fn root_schema_builder() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription).enable_federation()
}

/// The root query of the service
#[derive(Debug, Clone, Default)]
pub struct Query;

#[ComplexObject]
impl Session {
    /// Fetches all the data collected during a session with pagination
    async fn data_collection(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> async_graphql::Result<Connection<usize, DataCollection, EmptyFields, EmptyFields>> {
        let database = ctx.data::<DatabaseConnection>()?;
        let mut start: usize = after
            .map(|after| after.parse::<usize>().ok().unwrap_or(0))
            .unwrap_or(0);
        let mut end: usize = before
            .map(|before| before.parse::<usize>().ok().unwrap_or(usize::MAX))
            .unwrap_or(usize::MAX);

        if let Some(first) = first {
            end = (start + first as usize).min(end);
        }
        if let Some(last) = last {
            start = if last as usize > end - start {
                end
            } else {
                end - last as usize
            };
        }

        let limit: u64 = match end.checked_sub(start) {
            Some(diff) => diff as u64,
            None => return Err(async_graphql::Error::new("Pagination limit overflow")),
        };

        let data_collection_entities = data_collection::Entity::find()
            .filter(data_collection::Column::Sessionid.eq(self.id))
            .limit(limit)
            .offset(start as u64)
            .all(database)
            .await?;

        let data_collections: Vec<DataCollection> = data_collection_entities
            .into_iter()
            .map(DataCollection::from)
            .collect::<Vec<_>>();

        let mut connection: Connection<usize, DataCollection> =
            Connection::new(start > 0, end < usize::MAX);
        connection
            .edges
            .extend(
                data_collections
                    .into_iter()
                    .enumerate()
                    .map(|(index, data_collection)| {
                        Edge::with_additional_fields(index + start, data_collection, EmptyFields)
                    }),
            );

        Ok(connection)
    }
}

#[Object]
impl Query {
    /// Reference sessions resolver for the router
    #[graphql(entity)]
    async fn router_sessions(&self, id: i32) -> Session {
        Session { id }
    }
}

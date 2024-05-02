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
        first: u64,
    ) -> async_graphql::Result<
        Connection<OpaqueCursor<u32>, DataCollection, EmptyFields, EmptyFields>,
    > {
        let database = ctx.data::<DatabaseConnection>()?;
        let mut query =
            data_collection::Entity::find().filter(data_collection::Column::Sessionid.eq(self.id));

        if let Some(after) = after {
            if let Ok(cursor_value) = OpaqueCursor::<i32>::decode_cursor(&after) {
                query = query.filter(data_collection::Column::DataCollectionId.gt(cursor_value.0));
            } else {
                return Err(async_graphql::Error::new("Cursor not valid"));
            }
        }

        let limit = first.min(100);
        let mut data_collections = query
            .limit(limit + 1)
            .all(database)
            .await?
            .into_iter()
            .map(DataCollection::from)
            .collect::<Vec<_>>();

        let mut has_next_page = false;
        if data_collections.len() > limit as usize {
            has_next_page = true;
            data_collections.pop();
        }

        let mut connection = Connection::new(false, has_next_page);
        connection
            .edges
            .extend(data_collections.into_iter().map(|data_collection| {
                let cursor = OpaqueCursor(data_collection.id);
                Edge::with_additional_fields(cursor, data_collection, EmptyFields)
            }));

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

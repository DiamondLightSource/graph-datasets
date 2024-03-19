mod entity;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder};
use entity::DataCollection;
use models::data_collection;
use sea_orm::{DatabaseConnection, EntityTrait};

/// The GraphQL schema exposed by the service
pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

/// A schema builder for the service
pub fn root_schema_builder() -> SchemaBuilder<RootQuery, EmptyMutation, EmptySubscription> {
    Schema::build(RootQuery, EmptyMutation, EmptySubscription)
}

/// The root query of the service
#[derive(Debug, Clone, Default)]
pub struct RootQuery;

#[Object]
impl RootQuery {
    /// Retrieves all datasets collected during Sessions
    async fn datasets(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DataCollection>, async_graphql::Error> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(data_collection::Entity::find()
            .all(database)
            .await?
            .into_iter()
            .map(DataCollection::from)
            .collect())
    }
    /// Retrieves a single dataset collected during Sessions
    async fn dataset(
        &self,
        ctx: &Context<'_>,
        data_collection_id: u32,
    ) -> Result<Option<DataCollection>, async_graphql::Error> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(data_collection::Entity::find_by_id(data_collection_id)
            .one(database)
            .await?
            .map(DataCollection::from))
    }
}

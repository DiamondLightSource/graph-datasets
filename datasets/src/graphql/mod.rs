mod entity;

use async_graphql::{ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder};
use entity::{DataCollection, Session};
use models::data_collection;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

/// The GraphQL schema exposed by the service
pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

/// A schema builder for the service
pub fn root_schema_builder() -> SchemaBuilder<RootQuery, EmptyMutation, EmptySubscription> {
    Schema::build(RootQuery, EmptyMutation, EmptySubscription).enable_federation()
}

/// The root query of the service
#[derive(Debug, Clone, Default)]
pub struct RootQuery;

#[ComplexObject]
impl Session{
    async fn datasets(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<DataCollection>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(data_collection::Entity::find()
            .filter(data_collection::Column::Sessionid.eq(self.session_id))
            .all(database)
            .await?
            .into_iter()
            .map(DataCollection::from)
            .collect()
        )
    }
}

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

    #[graphql(entity)]
    async fn router_dataset(
        &self,
        ctx: &Context<'_>,
        data_collection_id: u32,
    ) -> Result<Option<DataCollection>, async_graphql::Error> {
        self.dataset(ctx, data_collection_id).await
    }

    #[graphql(entity)]
    async fn router_sessions(
        &self, 
        session_id: i32
    ) -> Session {
        Session {
            session_id,
        }
    }

}

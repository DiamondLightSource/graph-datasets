use async_graphql::{
    EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder, 
};

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
    /// Retrieves all data collected during Sessions
    async fn datasets(&self) -> &str {
        ""
    }
}

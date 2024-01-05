pub (crate) mod category;


#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(category::CategoryMutation);

use async_graphql::Enum;

pub(crate) mod category;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(category::CategoryMutation);

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum MutationType {
    Created,
    Updated,
    Deleted,
}

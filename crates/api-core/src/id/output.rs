use crate::Id;
use async_graphql::{parser::types::Field, *};

#[async_trait::async_trait]
impl OutputType for Id {
    fn type_name() -> std::borrow::Cow<'static, str> {
        "String".into()
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        <String as OutputType>::create_type_info(registry)
    }

    async fn resolve(
        &self,
        _: &ContextSelectionSet<'_>,
        _field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        Ok(Value::String(self.to_string()))
    }
}

#[async_trait::async_trait]
impl ContainerType for Id {
    async fn resolve_field(&self, _ctx: &Context<'_>) -> ServerResult<Option<Value>> {
        Ok(Some(Value::String(self.to_string())))
    }
}

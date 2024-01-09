use async_graphql::*;
use std::str::FromStr;

use crate::Id;

impl InputType for Id {
    type RawValueType = String;

    fn type_name() -> std::borrow::Cow<'static, str> {
        "String".into()
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        <String as OutputType>::create_type_info(registry)
    }

    fn parse(value: Option<Value>) -> InputValueResult<Self> {
        let value = value.unwrap_or_default();
        match value {
            Value::String(ref s) => {
                if let Ok(val) = Self::from_str(s) {
                    Ok(val)
                } else {
                    Err(InputValueError::expected_type(value))
                }
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        None
    }
}

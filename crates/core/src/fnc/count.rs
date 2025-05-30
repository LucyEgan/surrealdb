use super::args::Optional;
use crate::expr::value::Value;
use anyhow::Result;

pub fn count((Optional(arg),): (Optional<Value>,)) -> Result<Value> {
	Ok(arg
		.map(|val| match val {
			Value::Array(v) => v.iter().filter(|v| v.is_truthy()).count().into(),
			v => (v.is_truthy() as i64).into(),
		})
		.unwrap_or_else(|| 1.into()))
}

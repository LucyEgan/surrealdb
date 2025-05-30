use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::fmt;

#[revisioned(revision = 1)]
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub struct Explain(pub bool);

impl fmt::Display for Explain {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("EXPLAIN")?;
		if self.0 {
			f.write_str(" FULL")?;
		}
		Ok(())
	}
}

impl From<Explain> for crate::expr::Explain {
	fn from(v: Explain) -> Self {
		Self(v.0)
	}
}
impl From<crate::expr::Explain> for Explain {
	fn from(v: crate::expr::Explain) -> Self {
		Self(v.0)
	}
}

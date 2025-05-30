use reblessive::Stk;

use crate::{
	sql::{SqlValues, statements::UpdateStatement},
	syn::{
		parser::{ParseResult, Parser},
		token::t,
	},
};

impl Parser<'_> {
	pub async fn parse_update_stmt(&mut self, stk: &mut Stk) -> ParseResult<UpdateStatement> {
		let only = self.eat(t!("ONLY"));
		let what = SqlValues(self.parse_what_list(stk).await?);
		let with = self.try_parse_with()?;
		let data = self.try_parse_data(stk).await?;
		let cond = self.try_parse_condition(stk).await?;
		let output = self.try_parse_output(stk).await?;
		let timeout = self.try_parse_timeout()?;
		let parallel = self.eat(t!("PARALLEL"));
		let explain = self.try_parse_explain()?;

		Ok(UpdateStatement {
			only,
			what,
			with,
			data,
			cond,
			output,
			timeout,
			parallel,
			explain,
		})
	}
}

use crate::dbs::result::Results;
use crate::dbs::{Iterable, Statement};
use crate::idx::planner::RecordStrategy;

const TARGET: &str = "surrealdb::core::dbs::slow_log";

pub(super) struct SlowLog {
	// pub(super) do_iterate: bool,
	// pub(super) explanation: Option<Explanation>,
}

impl SlowLog {
	pub(super) fn new(
		// ctx: &Context,
		stm: &Statement<'_>,
		iterables: &Vec<Iterable>,
		// results: &Results,
	) -> Self {
		//query_log_level = none, slow, unindexed, all
		// let query_log_level = 4;

		for iter in iterables {
			match iter {
				Iterable::Value(_v) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Value)");
				}
				Iterable::Yield(_t) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Yield)");
				}
				Iterable::Thing(_t) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Thing)");
				}
				Iterable::Defer(_t) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Defer)");
				}
				Iterable::Edges(_e) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Edges)");
				}
				Iterable::Table(_t, rs) => {
					match rs {
						RecordStrategy::Count => {
							//query_log_level unindexed
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Table Count)");
						}
						RecordStrategy::KeysOnly => {
							//query_log_level unindexed
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Table Keys)");
						}
						RecordStrategy::KeysAndValues => {
							//query_log_level unindexed
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Table)");
						}
					};
				}
				Iterable::Range(_tb, _r, rs) => {
					match rs {
						RecordStrategy::Count => {
							//query_log_level all
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Range Count)");
						}
						RecordStrategy::KeysOnly => {
							//query_log_level all
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Range Keys)");
						}
						RecordStrategy::KeysAndValues => {
							//query_log_level all
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Range)");
						}
					};
				}
				Iterable::Mergeable(_t, _v) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Mergeable)");
				}
				Iterable::Relatable(_t1, _t2, _t3, None) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Relatable)");
				}
				Iterable::Relatable(_t1, _t2, _t3, Some(_v)) => {
					//query_log_level all
					trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Relatable)");
				}
				Iterable::Index(_t, _ir, rs) => {
					match rs {
						RecordStrategy::Count => {
							//query_log_level all
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Index Count)");
						}
						RecordStrategy::KeysOnly => {
							//query_log_level all
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Index Keys)");
						}
						RecordStrategy::KeysAndValues => {
							//query_log_level all
							trace!(target: TARGET, statement = %stm.to_string().replace("\n", "\\n"), "QueryMethod(Iterate Index)");
						}
					};
				}
			}
		}

		Self {}
	}
}

impl SlowLog {
	pub(crate) fn complete(&self, _results: &Results) {
		// todo!()
	}
}
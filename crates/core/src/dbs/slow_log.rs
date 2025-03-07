use crate::dbs::{Iterable};
use crate::idx::planner::RecordStrategy;

pub(super) fn slow_log(iterables: &Vec<Iterable>) -> String {
        iterables.iter()
                .map(|iter| {
                        match iter {
                                Iterable::Value(_v) => {
                                        //query_log_level all
                                        return "Value";
                                }
                                Iterable::Yield(_t) => {
                                        //query_log_level all
                                        return "Yield";
                                }
                                Iterable::Thing(_t) => {
                                        //query_log_level all
                                        return "Thing";
                                }
                                Iterable::Defer(_t) => {
                                        //query_log_level all
                                        return "Defer";
                                }
                                Iterable::Edges(_e) => {
                                        //query_log_level all
                                        return "Edges";
                                }
                                Iterable::Table(_t, rs, _sc) => {
                                        match rs {
                                                RecordStrategy::Count => {
                                                        //query_log_level unindexed
                                                        return "Table Count";
                                                }
                                                RecordStrategy::KeysOnly => {
                                                        //query_log_level unindexed
                                                        return "Table Keys";
                                                }
                                                RecordStrategy::KeysAndValues => {
                                                        //query_log_level unindexed
                                                        return "Table";
                                                }
                                        };
                                }
                                Iterable::Range(_tb, _r, rs, _sc) => {
                                        match rs {
                                                RecordStrategy::Count => {
                                                        //query_log_level all
                                                        return "Range Count";
                                                }
                                                RecordStrategy::KeysOnly => {
                                                        //query_log_level all
                                                        return "Range Keys";
                                                }
                                                RecordStrategy::KeysAndValues => {
                                                        //query_log_level all
                                                        return "Range";
                                                }
                                        };
                                }
                                Iterable::Mergeable(_t, _v) => {
                                        //query_log_level all
                                        return "Mergeable";
                                }
                                Iterable::Relatable(_t1, _t2, _t3, None) => {
                                        //query_log_level all
                                        return "Relatable";
                                }
                                Iterable::Relatable(_t1, _t2, _t3, Some(_v)) => {
                                        //query_log_level all
                                        return "Relatable";
                                }
                                Iterable::Index(_t, _ir, rs) => {
                                        match rs {
                                                RecordStrategy::Count => {
                                                        //query_log_level all
                                                        return "Index Count";
                                                }
                                                RecordStrategy::KeysOnly => {
                                                        //query_log_level all
                                                        return "Index Keys";
                                                }
                                                RecordStrategy::KeysAndValues => {
                                                        //query_log_level all
                                                        return "Index";
                                                }
                                        };
                                }
                        }
                })
                .collect::<Vec<_>>()
                .join(",")
}

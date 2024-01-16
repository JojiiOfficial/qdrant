use schemars::JsonSchema;
use segment::types::ShardKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[serde(untagged)]
pub enum ShardKeySelector {
    ShardKey(ShardKey),
    ShardKeys(Vec<ShardKey>),
    // ToDo: select by pattern
}

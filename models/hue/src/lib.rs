mod model;

use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use model::QueryRoot;

pub type HueSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct Hue {}

impl Hue {
    pub fn new() -> Self {
        Self {}
    }
}

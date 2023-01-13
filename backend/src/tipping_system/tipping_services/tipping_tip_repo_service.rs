use async_trait::async_trait;
use futures::stream::TryStreamExt;
use std::sync::Arc;

#[async_trait]
pub trait TipRepoServiceTrait {}

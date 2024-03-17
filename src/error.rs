use std::sync::PoisonError;

use taffy::NodeId;
use thiserror::Error;

pub type TaffyResult<T> = Result<T, TaffyError>;

#[derive(Error, Debug)]
pub enum TaffyError {
    #[error("lock poisoned")]
    Poison,
    #[error("child index out of bounds(parent: {parent:?}, child_index: {child_index}, child_count: {child_count})")]
    ChildIndexOutOfBounds {
        parent: NodeId,
        child_index: u64,
        child_count: u64,
    },
    #[error("child index out of bounds (node_id: {node_id:?})")]
    InvalidParentNode { node_id: NodeId },
    #[error("child index out of bounds (node_id: {node_id:?})")]
    InvalidChildNode { node_id: NodeId },
    #[error("child index out of bounds (node_id: {node_id:?})")]
    InvalidInputNode { node_id: NodeId },
}

impl<T> From<PoisonError<T>> for TaffyError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poison
    }
}

impl From<taffy::TaffyError> for TaffyError {
    fn from(error: taffy::TaffyError) -> Self {
        match error {
            taffy::TaffyError::ChildIndexOutOfBounds {
                parent,
                child_index,
                child_count,
            } => Self::ChildIndexOutOfBounds {
                parent,
                child_index: child_index as u64,
                child_count: child_count as u64,
            },
            taffy::TaffyError::InvalidParentNode(node_id) => Self::InvalidParentNode { node_id },
            taffy::TaffyError::InvalidChildNode(node_id) => Self::InvalidChildNode { node_id },
            taffy::TaffyError::InvalidInputNode(node_id) => Self::InvalidInputNode { node_id },
        }
    }
}

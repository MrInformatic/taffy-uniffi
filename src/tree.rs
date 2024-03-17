use std::sync::{Arc, RwLock};

use taffy::{Layout, NodeId, Size};

use crate::{AvailableSpace, Convert, Style, TaffyResult};

pub struct TaffyTree(RwLock<taffy::TaffyTree>);

impl TaffyTree {
    pub fn new() -> Self {
        Self(RwLock::new(taffy::TaffyTree::new()))
    }

    pub fn with_capacity(capacity: u64) -> Self {
        Self(RwLock::new(taffy::TaffyTree::with_capacity(
            capacity as usize,
        )))
    }

    pub fn get_child_at_index(&self, parent: NodeId, child_index: u64) -> TaffyResult<NodeId> {
        Ok(self
            .0
            .read()?
            .child_at_index(parent, child_index as usize)?)
    }

    pub fn get_children(&self, parent: NodeId) -> TaffyResult<std::vec::Vec<NodeId>> {
        Ok(self.0.read()?.children(parent)?)
    }

    pub fn clear(&self) -> TaffyResult<()> {
        Ok(self.0.write()?.clear())
    }

    pub fn compute_layout(
        &self,
        node: NodeId,
        available_space: Size<AvailableSpace>,
    ) -> TaffyResult<()> {
        Ok(self
            .0
            .write()?
            .compute_layout(node, available_space.convert())?)
    }

    pub fn compute_layout_with_measure(
        &self,
        node_id: NodeId,
        available_space: Size<AvailableSpace>,
        measure_function: Box<dyn MeasureFunction>,
    ) -> TaffyResult<()> {
        Ok(self.0.write()?.compute_layout_with_measure(
            node_id,
            available_space.convert(),
            |known_size, avaliable_space, node, _| measure_function.measure(known_size, avaliable_space.convert(), node),
        )?)
    }

    pub fn is_dirty(&self, node: NodeId) -> TaffyResult<bool> {
        Ok(self.0.write()?.dirty(node)?)
    }

    pub fn disable_rounding(&self) -> TaffyResult<()> {
        Ok(self.0.write()?.disable_rounding())
    }

    pub fn enable_rounding(&self) -> TaffyResult<()> {
        Ok(self.0.write()?.enable_rounding())
    }

    pub fn get_node_context(&self, node: NodeId) -> TaffyResult<()> {
        Ok(self.0.read()?.get_node_context(node).cloned().unwrap_or(()))
    }

    pub fn insert_child_at_index(
        &self,
        parent: NodeId,
        child_index: u64,
        child: NodeId,
    ) -> TaffyResult<()> {
        Ok(self
            .0
            .write()?
            .insert_child_at_index(parent, child_index as usize, child)?)
    }

    pub fn get_layout(&self, node: NodeId) -> TaffyResult<Layout> {
        Ok(self.0.read()?.layout(node)?.to_owned())
    }

    pub fn mark_dirty(&self, node: NodeId) -> TaffyResult<()> {
        Ok(self.0.write()?.mark_dirty(node)?)
    }

    pub fn new_leaf(&self, layout: std::sync::Arc<Style>) -> TaffyResult<NodeId> {
        Ok(self.0.write()?.new_leaf(layout.0.read()?.clone())?)
    }

    pub fn new_leaf_with_context(&self, layout: std::sync::Arc<Style>) -> TaffyResult<NodeId> {
        Ok(self
            .0
            .write()?
            .new_leaf_with_context(layout.0.read()?.clone(), ())?)
    }

    pub fn new_with_children(
        &self,
        layout: std::sync::Arc<Style>,
        children: std::vec::Vec<NodeId>,
    ) -> TaffyResult<NodeId> {
        Ok(self
            .0
            .write()?
            .new_with_children(layout.0.read()?.clone(), &children)?)
    }

    pub fn get_parent(&self, child_id: NodeId) -> TaffyResult<std::option::Option<NodeId>> {
        Ok(self.0.read()?.parent(child_id))
    }

    pub fn print_tree(&self, root: NodeId) -> TaffyResult<()> {
        Ok(self.0.write()?.print_tree(root))
    }

    pub fn remove(&self, node: NodeId) -> TaffyResult<NodeId> {
        Ok(self.0.write()?.remove(node)?)
    }

    pub fn remove_child(&self, parent: NodeId, child: NodeId) -> TaffyResult<NodeId> {
        Ok(self.0.write()?.remove_child(parent, child)?)
    }

    pub fn remove_child_at_index(&self, parent: NodeId, child_index: u64) -> TaffyResult<NodeId> {
        Ok(self
            .0
            .write()?
            .remove_child_at_index(parent, child_index as usize)?)
    }

    pub fn replace_child_at_index(
        &self,
        parent: NodeId,
        child_index: u64,
        new_child: NodeId,
    ) -> TaffyResult<NodeId> {
        Ok(self
            .0
            .write()?
            .replace_child_at_index(parent, child_index as usize, new_child)?)
    }

    pub fn set_children(&self, parent: NodeId, children: std::vec::Vec<NodeId>) -> TaffyResult<()> {
        Ok(self.0.write()?.set_children(parent, &children)?)
    }

    pub fn set_node_context(&self, node: NodeId) -> TaffyResult<()> {
        Ok(self.0.write()?.set_node_context(node, Some(()))?)
    }

    pub fn set_style(&self, node: NodeId, style: std::sync::Arc<Style>) -> TaffyResult<()> {
        Ok(self.0.write()?.set_style(node, style.0.read()?.clone())?)
    }

    pub fn get_style(&self, node: NodeId) -> TaffyResult<std::sync::Arc<Style>> {
        Ok(Arc::new(self.0.read()?.style(node)?.to_owned().into()))
    }

    pub fn get_total_node_count(&self) -> TaffyResult<u64> {
        Ok(self.0.read()?.total_node_count() as u64)
    }
}

pub trait MeasureFunction {
    fn measure(
        &self,
        known_size: Size<Option<f32>>,
        avaliable_space: Size<AvailableSpace>,
        node: NodeId,
    ) -> Size<f32>;
}

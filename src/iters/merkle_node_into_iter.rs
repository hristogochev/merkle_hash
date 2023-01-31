use std::collections::btree_set::IntoIter;

use crate::components::merkle_item::MerkleItem;
use crate::tree::merkle_node::MerkleNode;

/// Owned node iterator
#[derive(Default)]
pub struct MerkleNodeIntoIter {
    value: Option<MerkleItem>,
    children: Option<IntoIter<MerkleNode>>,
    parent: Option<Box<MerkleNodeIntoIter>>,
}

impl MerkleNodeIntoIter {
    pub fn new(
        value: MerkleItem,
        children: IntoIter<MerkleNode>,
        parent: Option<Box<MerkleNodeIntoIter>>,
    ) -> Self {
        Self {
            value: Some(value),
            children: Some(children),
            parent,
        }
    }
}

impl Iterator for MerkleNodeIntoIter {
    type Item = MerkleItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.value.take() {
            return Some(value);
        }
        if let Some(first_child) = self.children.as_mut().and_then(|children| children.next()) {
            return if first_child.children.is_empty() {
                Some(first_child.item)
            } else {
                *self = MerkleNodeIntoIter::new(
                    first_child.item,
                    first_child.children.into_iter(),
                    Some(Box::new(std::mem::take(self))),
                );
                self.next()
            };
        } else if let Some(parent) = self.parent.take() {
            *self = *parent;
            return self.next();
        }
        None
    }
}

impl IntoIterator for MerkleNode {
    type Item = MerkleItem;

    type IntoIter = MerkleNodeIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MerkleNodeIntoIter::new(self.item, self.children.into_iter(), None)
    }
}

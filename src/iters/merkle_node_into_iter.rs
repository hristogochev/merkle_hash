use std::collections::btree_set::IntoIter;

use crate::components::merkle_item::MerkleItem;
use crate::tree::merkle_node::MerkleNode;

/// Owned node iterator
#[derive(Default)]
pub struct MerkleNodeIntoIter<const N: usize> {
    value: Option<MerkleItem<N>>,
    children: Option<IntoIter<MerkleNode<N>>>,
    parent: Option<Box<MerkleNodeIntoIter<N>>>,
}

impl<const N: usize> MerkleNodeIntoIter<N> {
    pub fn new(
        value: MerkleItem<N>,
        children: IntoIter<MerkleNode<N>>,
        parent: Option<Box<MerkleNodeIntoIter<N>>>,
    ) -> Self {
        Self {
            value: Some(value),
            children: Some(children),
            parent,
        }
    }
}

impl<const N:usize> Iterator for MerkleNodeIntoIter<N> {
    type Item = MerkleItem<N>;

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

impl<const N:usize> IntoIterator for MerkleNode<N> {
    type Item = MerkleItem<N>;

    type IntoIter = MerkleNodeIntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        MerkleNodeIntoIter::new(self.item, self.children.into_iter(), None)
    }
}

use std::collections::btree_set::Iter;
use std::iter::FusedIterator;

use crate::components::merkle_item::MerkleItem;
use crate::tree::merkle_node::MerkleNode;

/// Node iterator
#[derive(Default)]
pub struct MerkleNodeIter<'a> {
    value: Option<&'a MerkleItem>,
    children: Option<Iter<'a, MerkleNode>>,
    parent: Option<Box<MerkleNodeIter<'a>>>,
}

impl<'a> MerkleNodeIter<'a> {
    pub fn new(
        value: &'a MerkleItem,
        children: Iter<'a, MerkleNode>,
        parent: Option<Box<MerkleNodeIter<'a>>>,
    ) -> Self {
        Self {
            value: Some(value),
            children: Some(children),
            parent,
        }
    }
}

impl<'a> Iterator for MerkleNodeIter<'a> {
    type Item = &'a MerkleItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.value.take() {
            return Some(value);
        }

        if let Some(first_child) = self.children.as_mut().and_then(|children| children.next()) {
            return if first_child.children.is_empty() {
                Some(&first_child.item)
            } else {
                *self = MerkleNodeIter::new(
                    &first_child.item,
                    first_child.children.iter(),
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

impl MerkleNode {
    /// Returns an iterator over each file and directory descendant of the current node
    pub fn iter(&self) -> MerkleNodeIter<'_> {
        MerkleNodeIter::new(&self.item, self.children.iter(), None)
    }
}

impl<'a> IntoIterator for &'a MerkleNode {
    type Item = &'a MerkleItem;

    type IntoIter = MerkleNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FusedIterator for MerkleNodeIter<'_> {}

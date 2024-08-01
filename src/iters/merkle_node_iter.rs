use std::collections::btree_set::Iter;
use std::iter::FusedIterator;

use crate::components::merkle_item::MerkleItem;
use crate::tree::merkle_node::MerkleNode;

/// Node iterator
#[derive(Default)]
pub struct MerkleNodeIter<'a, const N: usize> {
    value: Option<&'a MerkleItem<N>>,
    children: Option<Iter<'a, MerkleNode<N>>>,
    parent: Option<Box<MerkleNodeIter<'a, N>>>,
}

impl<'a, const N: usize> MerkleNodeIter<'a, N> {
    pub fn new(
        value: &'a MerkleItem<N>,
        children: Iter<'a, MerkleNode<N>>,
        parent: Option<Box<MerkleNodeIter<'a, N>>>,
    ) -> Self {
        Self {
            value: Some(value),
            children: Some(children),
            parent,
        }
    }
}

impl<'a,const N: usize> Iterator for MerkleNodeIter<'a,N> {
    type Item = &'a MerkleItem<N>;

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

impl<const N: usize> MerkleNode<N> {
    /// Returns an iterator over each file and directory descendant of the current node
    pub fn iter(&self) -> MerkleNodeIter<'_,N> {
        MerkleNodeIter::new(&self.item, self.children.iter(), None)
    }
}

impl<'a,const N: usize> IntoIterator for &'a MerkleNode<N> {
    type Item = &'a MerkleItem<N>;

    type IntoIter = MerkleNodeIter<'a,N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a,const N: usize> FusedIterator for MerkleNodeIter<'a,N> {}

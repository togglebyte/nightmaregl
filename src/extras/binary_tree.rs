use crate::pixels::{Pixel, Pixels};
use crate::texture::{Format, Texture};
use crate::{Position, Size, Sprite};
use std::ptr::NonNull;

// -----------------------------------------------------------------------------
//     - Node iterator -
// -----------------------------------------------------------------------------
pub struct NodeIter<'a, T> {
    inner: Vec<&'a Node<T>>,
}

impl<'a, T> NodeIter<'a, T> {
    pub fn new(root: &'a Node<T>) -> Self {
        Self { inner: vec![root] }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let el = self.inner.pop()?;

        if let Some(ref el) = el.left() {
            self.inner.push(el)
        }
        if let Some(ref el) = el.right() {
            self.inner.push(el)
        }

        Some(&el.inner)
    }
}

pub struct NodeIterMut<'a, T> {
    inner: Vec<&'a mut Node<T>>,
}

impl<'a, T> NodeIterMut<'a, T> {
    pub fn new(root: &'a mut Node<T>) -> Self {
        Self { inner: vec![root] }
    }
}

impl<'a, T> Iterator for NodeIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let el = self.inner.pop()?;

        if let Some(ref mut left) = el.left {
            let left = unsafe { left.as_mut() };
            self.inner.push(left);
        }

        if let Some(ref mut right) = el.right {
            let right = unsafe { right.as_mut() };
            self.inner.push(right);
        }

        Some(&mut el.inner)
    }
}

// -----------------------------------------------------------------------------
//     - Node and container -
// -----------------------------------------------------------------------------
pub struct Node<T> {
    inner: T,
    left: Option<NonNull<Node<T>>>,
    right: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            left: None,
            right: None,
        }
    }

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter::new(self)
    }

    pub fn iter_mut(&mut self) -> NodeIterMut<T> {
        NodeIterMut::new(self)
    }

    pub fn as_ref(&self) -> &T {
        &self.inner
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn insert_left(&mut self, val: T) {
        let val = Box::new(Node::new(val));
        let val = Box::leak(val);
        self.left = NonNull::new(val);
    }

    pub fn insert_right(&mut self, val: T) {
        let val = Box::new(Node::new(val));
        let val = Box::leak(val);
        self.right = NonNull::new(val);
    }

    pub fn left(&self) -> Option<&Node<T>> {
        match self.left {
            None => None,
            Some(ref left) => unsafe { Some(left.as_ref()) },
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut Node<T>> {
        match self.left {
            None => None,
            Some(ref mut left) => unsafe { Some(left.as_mut()) },
        }
    }

    pub fn right(&self) -> Option<&Node<T>> {
        match self.right {
            None => None,
            Some(ref right) => unsafe { Some(right.as_ref()) },
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut Node<T>> {
        match self.right {
            None => None,
            Some(ref mut right) => unsafe { Some(right.as_mut()) },
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let left = self.left.take();
        let right = self.right.take();
        if let Some(left) = left {
            unsafe { Box::from_raw(left.as_ptr()) };
        }
        if let Some(right) = right {
            unsafe { Box::from_raw(right.as_ptr()) };
        }
    }
}

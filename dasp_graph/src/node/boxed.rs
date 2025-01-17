use crate::{Buffer, Input, Node};
use core::fmt;
use core::ops::{Deref, DerefMut};

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
pub struct BoxedNode<I>(pub Box<dyn Node<I>>);

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
///
/// Useful when the ability to send nodes from one thread to another is required. E.g. this is
/// common when initialising nodes or the audio graph itself on one thread before sending them to
/// the audio thread.
pub struct BoxedNodeSend<I>(pub Box<dyn Node<I> + Send>);

impl<I> BoxedNode<I> {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new<T>(node: T) -> Self
    where
        T: 'static + Node<I>,
    {
        Self::from(Box::new(node))
    }
}

impl<I> BoxedNodeSend<I> {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new<T>(node: T) -> Self
    where
        T: 'static + Node<I> + Send,
    {
        Self::from(Box::new(node))
    }
}

impl<I> Node<I> for BoxedNode<I> {
    fn process(&mut self, inputs: &[Input<I>], output: &mut [Buffer]) {
        self.0.process(inputs, output)
    }
}

impl<I> Node<I> for BoxedNodeSend<I> {
    fn process(&mut self, inputs: &[Input<I>], output: &mut [Buffer]) {
        self.0.process(inputs, output)
    }
}

impl<T, I> From<Box<T>> for BoxedNode<I>
where
    T: 'static + Node<I>,
{
    fn from(n: Box<T>) -> Self {
        BoxedNode(n as Box<dyn Node<I>>)
    }
}

impl<T, I> From<Box<T>> for BoxedNodeSend<I>
where
    T: 'static + Node<I> + Send,
{
    fn from(n: Box<T>) -> Self {
        BoxedNodeSend(n as Box<dyn Node<I> + Send>)
    }
}

impl<I> Into<Box<dyn Node<I>>> for BoxedNode<I> {
    fn into(self) -> Box<dyn Node<I>> {
        self.0
    }
}

impl<I> Into<Box<dyn Node<I> + Send>> for BoxedNodeSend<I> {
    fn into(self) -> Box<dyn Node<I> + Send> {
        self.0
    }
}

impl<I> fmt::Debug for BoxedNode<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNode").finish()
    }
}

impl<I> fmt::Debug for BoxedNodeSend<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNodeSend").finish()
    }
}

impl<I> Deref for BoxedNode<I> {
    type Target = Box<dyn Node<I>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I> Deref for BoxedNodeSend<I> {
    type Target = Box<dyn Node<I> + Send>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for BoxedNode<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I> DerefMut for BoxedNodeSend<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#![allow(dead_code)]
pub type EdgeIndex = usize;

#[derive(Debug, Clone)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<T>>,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    edges: Vec<Box<Node<T>>>,
    visited: bool,
}

#[derive(Debug, Clone)]
pub struct Edge<T> {
    des_index: EdgeIndex,
    val: T,
}

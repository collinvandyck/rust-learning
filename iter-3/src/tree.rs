pub enum Tree<K, V> {
    Empty,
    NonEmpty(Node<K, V>),
}

pub struct Node<K, V> {
    key: K,
    val: V,
    children: Vec<Node<K, V>>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
    V: PartialEq,
{
}

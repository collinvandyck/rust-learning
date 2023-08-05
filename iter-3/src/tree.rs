pub enum Tree<K, V> {
    Empty,
    NonEmpty(Box<Node<K, V>>),
}

pub struct Node<K, V> {
    key: K,
    val: V,
}

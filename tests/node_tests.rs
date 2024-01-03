use nodex::node::Node;

#[test]
fn test_node_from_json() {
    let node: Node = Node::from_json("tests/test_files/node_1.json");
    println!("{:?}", node)
}

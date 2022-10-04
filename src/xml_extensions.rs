pub fn extract_text_from_node(node: roxmltree::Node) -> Vec<String> {
    let mut vec = Vec::new();                
    for node in node.descendants().filter(|n| n.is_text()) {
        match node.text() {
            Some(t) => vec.push(String::from(t)),
            None => (),
        }
    }
    vec
}

pub fn extract_text_from_nodes(nodes: Vec<roxmltree::Node>) -> Vec<String> {
    nodes.iter().flat_map(|n| extract_text_from_node(*n)).collect()
}
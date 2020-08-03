use std::collections::HashMap;

pub struct Node {
    // data common to all notes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

pub enum NodeType { 
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs
        })
    }
}

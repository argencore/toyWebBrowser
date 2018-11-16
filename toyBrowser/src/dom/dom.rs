//
//
///! This module implements the Document Object Model (DOM).
///! The DOM is the programming interface for HTML and XML.
///! It represents the documents as nodes that can be used by
///! programming languages.


use std::collections::{HashMap};

/// The Node struct contains its children and type, the type
/// being the object from the HTML or XML being represented
struct Node{
    // data common to all nodes: 
    children: Vec<Node>,

    //data specific to each node type
    node_type: NodeType, 
}

/// The enum NodeType is used to create the different kinds of 
/// nodes that can be created and the data they store. This is
/// a simplified implementation
enum NodeType{
    Text(String),
    Element(ElementData),
}

/// represents the data an element contains, in this case that 
/// is only a tag name and a hash map string -> string
struct ElementData{
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String,String>;
/** fn text
 The function text converts text into a text node
 # Arguments
 
 * `data` - a string containing the text data
 
 # Outputs
 
 * `Node` - a node containting the text data of Text type
*/
fn text(data: String) -> Node{
    Node{
        children: Vec::new(), 
        node_type: NodeType::Text(data),
    }
}
/** fn elem 
    The function elem creates a Node with the Element enum type
    # Arguments
 
    * `name` - the tag_name of the element being created
    * `attrs` - the AttrMap(HashMap) of string data
    * `children` - the child nodes that this node will inherit
 
    # Output
 
    * `Node` - a node containing the element data of Element type
 */ 
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node{
    Node{
        children: children,
        node_type: NodeType::Element(ElementData{
            tag_name: name,
            attributes: attrs,
        })
    }
}

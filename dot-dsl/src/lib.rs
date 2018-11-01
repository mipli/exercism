pub mod graph {
    pub mod graph_items {
        type NodeId = String;
        pub mod edge {
            use std::collections::{HashMap};

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub attrs: HashMap<String, String>,
                pub start: super::NodeId,
                pub end: super::NodeId
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Edge {
                        attrs: HashMap::default(),
                        start: start.to_string(),
                        end: end.to_string()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|&(key, val)| {
                        self.attrs.insert(key.to_string(), val.to_string());
                    });
                    self
                }
            }
        }

        pub mod node {
            use std::collections::{HashMap};

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub attrs: HashMap<String, String>,
                pub name: super::NodeId
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        attrs: HashMap::default(),
                        name: name.to_string()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|&(key, val)| {
                        self.attrs.insert(key.to_string(), val.to_string());
                    });
                    self
                }
            }
        }
    }

    use std::collections::{HashMap};

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;

    pub struct Graph {
        pub attrs: HashMap<String, String>,
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                attrs: HashMap::default(),
                edges: vec![],
                nodes: vec![]
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|&(key, val)| {
                self.attrs.insert(key.to_string(), val.to_string());
            });
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn is_valid(self) -> bool {
            let has_node = |name: &String| self.nodes.iter().find(|n| n.name == *name).is_some();
            self.edges.iter().all(|edge| {
                has_node(&edge.start) && has_node(&edge.end)
            })
        }
    }
}

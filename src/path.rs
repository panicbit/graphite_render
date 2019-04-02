use std::fmt;
use itertools::Itertools;

pub struct Path {
    path: String,
    num_nodes: usize,
    alias_by_node: Option<usize>,
}

impl Path {
    pub fn new(path: &str) -> Self {
        Path {
            path: String::new(),
            num_nodes: 0,
            alias_by_node: None,
        }
        .dot(path)
    }

    pub fn dot(mut self, path: &str) -> Self {
        let nodes = path.trim().split(".").filter(|node| !node.is_empty());

        for node in nodes {
            if !self.path.is_empty() {
                self.path.push('.');
            }

            self.path.push_str(node);
            self.num_nodes += 1;
        }

        self
    }

    pub fn values<V: fmt::Display>(self, values: &[V]) -> Self {
        let values = values.iter().join(",");
        let this = self.dot(&format!("{{{}}}", values));

        this
    }

    pub fn alias_by_current_node(mut self) -> Self {
        if self.num_nodes > 0 {
            self.alias_by_node = Some(self.num_nodes - 1);
        }

        self
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)?;

        if let Some(index) = self.alias_by_node {
            write!(f, "|aliasByNode({})", index)?;
        }

        Ok(())
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Self {
        self
    }
}

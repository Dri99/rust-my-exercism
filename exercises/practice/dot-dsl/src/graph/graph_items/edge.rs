use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;

//use node::Node;
#[derive(Clone,Eq, PartialEq,Debug, Default)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub attrs: HashMap<String, String>
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        return Edge {from:from.to_string(), to: to.to_string(), attrs: HashMap::new() } ;
    }

    pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self {
        let hashmap: HashMap<String,String> = attrs.iter()
            .map(|(key, val)| {(key.to_string(),val.to_string())})
            .collect();
        return Edge{attrs: hashmap, ..self };
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}--{} [", self.from, self.to);
        for (i,attr) in self.attrs.iter().enumerate() {
            write!(f, " {}=\"{}\"", attr.0, attr.1);
            if i < self.attrs.len() -1 {
                write!(f, ",");
            }
        }
        write!(f, " ]");
        return Ok(());
    }
}
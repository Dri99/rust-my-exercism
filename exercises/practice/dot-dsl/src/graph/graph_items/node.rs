use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Clone,Debug,Eq, PartialEq,Default)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String,String>
}


impl Node{
    pub fn new(name: &str) -> Self{
        return Node {name: name.to_string(), attrs: HashMap::new()};
        //return Node::default();
    }

    pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self {
        let hashmap: HashMap<String,String> = attrs.iter()
            .map(|(key, val)| {(key.to_string(),val.to_string())})
            .collect();
        return Node{attrs: hashmap, ..self };
    }

    pub fn get_attr(&self, node: &str) -> Option<&str> {
        return self.attrs.get(node).map(|content| {content.as_str()});
        // return match self.attrs.get(node) {
        //     Some(res)=> Some(res.as_str()),
        //     None => None
        // };
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} [", self.name);
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


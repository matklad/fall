use std::collections::HashMap;
use std::sync::Mutex;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeType(pub u32);

pub const ERROR: NodeType = NodeType(0);
pub const WHITESPACE: NodeType = NodeType(1);


#[derive(Clone, Copy)]
pub struct NodeTypeInfo {
    pub name: &'static str
}

impl NodeType {
    pub fn register(self, info: NodeTypeInfo) {
        NODE_INFO.lock().unwrap().insert(self, info);
    }

    pub fn name(self) -> &'static str {
        self.info().name
    }

    fn info(self, ) -> NodeTypeInfo {
        let m = NODE_INFO.lock().unwrap();
        m.get(&self)
            .expect("Unregistered NodeType")
            .clone()
    }
}

impl ::std::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("NodeType(")?;
        let m = NODE_INFO.lock().unwrap();
        if let Some(n) = m.get(self) {
            f.write_str(n.name)?;
        } else {
            write!(f, "unregistered {}", self.0)?;
        }
        f.write_str(")")?;
        Ok(())
    }
}

lazy_static! {
    static ref NODE_INFO: Mutex<HashMap<NodeType, NodeTypeInfo>> = {
        let mut map = HashMap::new();
        map.insert(ERROR, NodeTypeInfo { name: "ERROR" });
        map.insert(WHITESPACE, NodeTypeInfo { name: "WHITESPACE" });
        Mutex::new(map)
    };
}

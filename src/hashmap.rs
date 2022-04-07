use hashbrown::HashMap;

use crate::controllers;

pub fn _create_hashmap() {
    let parsedconfig = controllers::_parse_configuration();
    println!("{:?}", parsedconfig.unwrap());
    let mut hashmap = HashMap::new();
    hashmap.insert("testes", "one");
}

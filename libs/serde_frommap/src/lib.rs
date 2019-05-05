
#[macro_use]
extern crate serde;

pub use self::err::{Error, Result};

pub mod err;
pub mod de;

use crate::de::from_map;
use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;

#[test]
fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        int: u32,
        boolean: bool,
    }

    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    hm.insert("int".into(), vec!["1".into()]);
    hm.insert("boolean".into(), vec!["false".into()]);

    let expected = Test {
        int: 1,
        boolean: false,
    };
    assert_eq!(expected, from_map(&hm).unwrap());
}


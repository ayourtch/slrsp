
#[macro_use]
extern crate serde;

pub use self::err::{Error, Result};

pub mod err;
pub mod de;

pub use crate::de::from_map;
use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;

#[test]
fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        int: u32,
        boolean: bool,
        int2: i32,
    }

    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    hm.insert("int".into(), vec!["1".into()]);
    hm.insert("int2".into(), vec!["-23".into()]);
    hm.insert("boolean".into(), vec!["false".into()]);

    let expected = Test {
        int: 1,
        int2: -23,
        boolean: false,
    };
    assert_eq!(expected, from_map(&hm).unwrap());
}


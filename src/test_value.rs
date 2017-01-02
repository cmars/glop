#![cfg(test)]

use super::value::*;

fn test_obj() -> Obj {
    [("apple".to_string(),
      Value::Object([("color".to_string(), Value::from_str("red")),
                     ("size".to_string(), Value::from_int(4))]
          .iter()
          .cloned()
          .collect())),
     ("pi".to_string(), Value::from_str("3.14"))]
        .iter()
        .cloned()
        .collect()
}

#[test]
fn get_l1_value_exist() {
    let o = test_obj();
    assert_eq!(Identifier::from_str("pi").get(&o),
               Some(&Value::Str("3.14".to_string())))
}

#[test]
fn get_l1_value_not_exist() {
    let o = test_obj();
    assert_eq!(Identifier::from_str("tau").get(&o), None)
}

#[test]
fn get_l2_value_exist() {
    let o = test_obj();
    assert_eq!(Identifier::from_str("apple.color").get(&o),
               Some(&Value::from_str("red")));
    assert_eq!(Identifier::from_str("apple.size").get(&o),
               Some(&Value::from_int(4)));
}

#[test]
fn get_ln_value_not_exist() {
    let o = test_obj();
    assert_eq!(Identifier::from_str("apple.core.seed.germ.cell.nucleus.chromosome").get(&o),
               None)
}

#[test]
fn set_change_value_exist() {
    let o = &mut test_obj();
    let id = Identifier::from_str("apple.color");
    assert_eq!(id.get(o), Some(&Value::Str("red".to_string())));
    id.set(o, Value::from_str("gold"));
    assert_eq!(id.get(o), Some(&Value::Str("gold".to_string())));
    assert_eq!(Identifier::from_str("apple.size").get(&o),
               Some(&Value::from_int(4)))
}

#[test]
fn set_change_flatten_value_exist() {
    let o = &mut test_obj();
    assert_eq!(Identifier::from_str("apple.color").get(o),
               Some(&Value::Str("red".to_string())));
    Identifier::from_str("apple").set(o, Value::from_str("manzana"));
    assert_eq!(Identifier::from_str("apple").get(o),
               Some(&Value::from_str("manzana")));
    assert_eq!(Identifier::from_str("apple.color").get(o), None);
    assert_eq!(Identifier::from_str("apple.size").get(&o), None);
}

#[test]
fn set_change_extrude_value_exist() {
    let o = &mut test_obj();
    assert_eq!(Identifier::from_str("apple.color").get(o),
               Some(&Value::Str("red".to_string())));
    Identifier::from_str("apple.color.r").set(o, Value::from_int(255));
    Identifier::from_str("apple.color.g").set(o, Value::from_int(0));
    Identifier::from_str("apple.color.b").set(o, Value::from_int(0));
    assert_eq!(Identifier::from_str("apple.color.r").get(o),
               Some(&Value::from_int(255)));
    assert_eq!(Identifier::from_str("apple.color.g").get(o),
               Some(&Value::from_int(0)));
    assert_eq!(Identifier::from_str("apple.color.b").get(o),
               Some(&Value::from_int(0)));
    assert_eq!(Identifier::from_str("apple.color").get(o),
               Some(&Value::from_obj([("r".to_string(), Value::from_int(255)),
                                      ("g".to_string(), Value::from_int(0)),
                                      ("b".to_string(), Value::from_int(0))]
                   .iter()
                   .cloned()
                   .collect())));
    assert_eq!(Identifier::from_str("apple.size").get(&o),
               Some(&Value::from_int(4)));
}

#[test]
fn unset_value_exist() {
    let o = &mut test_obj();
    let id = Identifier::from_str("apple.color");
    assert_eq!(id.get(o), Some(&Value::Str("red".to_string())));
    for _ in 0..2 {
        id.unset(o);
        assert_eq!(id.get(o), None);
        assert_eq!(Identifier::from_str("apple.size").get(&o),
                   Some(&Value::from_int(4)))
    }
}

#[test]
fn unset_value_not_exist() {
    let o = &mut test_obj();
    Identifier::from_str("nope").unset(o);
    Identifier::from_str("nope.nothing.to.see.here").unset(o);
    Identifier::from_str("apple.banana").unset(o);
    assert_eq!(Identifier::from_str("apple.size").get(&o),
               Some(&Value::from_int(4)))
}

#[test]
fn unset_parent_exist() {
    let o = &mut test_obj();
    assert_eq!(Identifier::from_str("apple.color").get(o),
               Some(&Value::Str("red".to_string())));
    Identifier::from_str("apple").unset(o);
    assert_eq!(Identifier::from_str("apple.color").get(o), None);
    assert_eq!(Identifier::from_str("apple.size").get(o), None);
    assert_eq!(Identifier::from_str("apple").get(o), None);
}

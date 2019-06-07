#[macro_export]

macro_rules! two  {
  () => { 1 + 1 }
}

macro_rules! calc  {
  (two) => { 1 + 1 };
  (three) => { 1 + 2 }
}

macro_rules! repeat_n_times {
    ($n:expr, $text:expr) => {(0..$n).map(|_| format!("{}", $text)).collect::<Vec<String>>()}
}

#[derive(PartialEq, Debug)]
struct Response(usize);


// Declare a function in a macro!
macro_rules! handler {
    ($i: ident, $body: block) => {
        fn $i () -> Response $body
    } 
}

use std::collections::BTreeSet;

macro_rules! set {
 ( $( $item:expr ),* ) => {
        {
            let mut s = BTreeSet::new();
            $(
                s.insert($item);
            )*
            s
        }
    };
}

mod helpers {
  #[macro_export]
  macro_rules! key_value {
  ( $cls: ty, $( $key:expr => $value:expr ),* ) => {
          {
              let mut s = <$cls>::new();
              $(
                  s.insert($key, $value);
              )*
              s
          }
      };
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::{BTreeMap, HashMap};


  #[test]
  fn test_two() {
      assert_eq!(2, two!());
  }

  #[test]
  fn test_calc() {
    assert_eq!(2, calc!(two));
    assert_eq!(3, calc!(three));
  }

  #[test]
  fn test_repeat() {
    assert_eq!(vec!["Hello", "Hello", "Hello"], repeat_n_times!(3, "Hello"));
  }

  #[test]
  fn test_req_handler() {
    handler!(status_handler, { Response(200)});
    assert_eq!(Response(200), status_handler());
  }

  #[test]
  fn test_set() {
    let actual = set!("a", "b", "c", "a");
    let mut desired = BTreeSet::new();
    desired.insert("a");
    desired.insert("b");
    desired.insert("c");
    assert_eq!(actual, desired);
  }

  #[test]
  fn test_kv() {
    let actual = key_value!(BTreeMap<&str, usize>, "hello" => 2, "world" => 1);
    let mut desired = BTreeMap::new();
    desired.insert("hello", 2);
    desired.insert("world", 1);
    assert_eq!(actual, desired);

    let actual = key_value!(HashMap<&str, usize>, "hello" => 2, "world" => 1);
    let mut desired = HashMap::new();
    desired.insert("hello", 2);
    desired.insert("world", 1);
    assert_eq!(actual, desired);
  }
}

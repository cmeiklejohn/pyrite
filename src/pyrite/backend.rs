#[desc = "Backend implementation for Pyrite"];
#[license = "MIT"];

use std::hashmap::HashMap;

struct MemoryBackend {
  reference: HashMap<~str, ~str>
}

impl MemoryBackend {
  fn new() -> MemoryBackend {
    MemoryBackend { reference: HashMap::new() }
  }

  fn get(&self, key: ~str) -> Option<~str> {
    self.reference.find_copy(&key)
  }

  fn put(&mut self, key: ~str, value: ~str) -> bool {
    self.reference.insert(key.clone(), value.clone())
  }
}

#[test]
fn test() {
  let mut backend: MemoryBackend = MemoryBackend::new();

  // Verify we can put an object.
  assert!(backend.put(~"key", ~"value"));

  // Verify we can retrieve the objecta after putting.
  match backend.get(~"key") {
    Some(value) => {
      assert!(~"value" == value)
    },
    None => {
      fail!("Key could not be found after writing!")
    }
  }

  // Verify we get an error for an object that isn't there.
  match backend.get(~"missing") {
    None => {
      assert!(true)
    },
    _ => {
      fail!("Found an object we didn't write!")
    }
  }
}

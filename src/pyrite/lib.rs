#[desc = "The Pyrite Rust package."];
#[license = "MIT"];

extern mod extra;

use std::hashmap::HashMap;
use extra::comm::DuplexStream;

enum BackendRequestMessage {
  BackendTerminate(),
  BackendGetRequest(~str),
  BackendStoreRequest(~str, ~str)
}

enum BackendResponseMessage {
  BackendGetFailedResponse(~str),
  BackendGetSuccessResponse(~str, ~str),
  BackendStoreFailedResponse(~str, ~str),
  BackendStoreSuccessResponse(~str, ~str)
}

/// Memory based backend for a simplistic key/value store.
fn backend(channel: &DuplexStream<BackendResponseMessage, BackendRequestMessage>) {
  let mut backend: HashMap<~str, ~str>;
  let mut message: BackendRequestMessage;

  backend = HashMap::new();

  loop {
    message = channel.recv();

    match message {
      BackendGetRequest(key) => {
        println(format!("Request to retrieve key {:s}", key));
        match backend.find_copy(&key) {
          Some(value) => {
            channel.send(BackendGetSuccessResponse(key, value));
          },
          _ => {
            channel.send(BackendGetFailedResponse(key));
          }
        }
      }
      BackendStoreRequest(key, value) => {
        println(format!("Request to store key/value pair {:s} => {:s}",
                        key, value));
        match backend.insert(key.clone(), value.clone()) {
          true => {
            channel.send(BackendStoreSuccessResponse(key, value))
          },
          false => {
            channel.send(BackendStoreFailedResponse(key, value))
          }
        };
      },
      BackendTerminate() => {
        println(format!("Terminating backend."));
        break
      }
    }
  }
}

// Perform a basic test of the key/value store.
fn main() {
  let (server, client) = DuplexStream();

  do spawn {
    backend(&server);
  }

  client.send(BackendStoreRequest(~"Ingmar Bergman", ~"The Silence"));
  client.send(BackendStoreRequest(~"Jean-Luc Godard", ~"Breathless"));

  client.send(BackendGetRequest(~"Jean-Luc Godard"));
  client.send(BackendGetRequest(~"Francois Truffaut"));

  do 3.times {
    match client.recv() {
      BackendGetFailedResponse(key) => {
        println(format!("Key {:s} retrieval failed!", key));
      },
      BackendGetSuccessResponse(key, value) => {
        println(format!("Key {:s} retrieval successful => {:s}!", key, value));
      },
      BackendStoreFailedResponse(key, _) => {
        println(format!("Key {:s} store failed!", key));
      },
      BackendStoreSuccessResponse(key, _) => {
        println(format!("Key {:s} stored successfully!", key));
      }
    }
  }

  client.send(BackendTerminate);
}

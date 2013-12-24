#[desc = "The Pyrite Rust package."];
#[license = "MIT"];

extern mod extra;

use std::hashmap::HashMap;

use std::cell::RefCell;

use std::io::{Writer, Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

use extra::comm::DuplexStream;

/// Backend implementations, providing a trait and a simplified memory
/// backend for testing.

trait Backend {
  fn new() -> Self;
  fn get(&self, key: ~str) -> Option<~str>;
  fn put(&mut self, key: ~str, value: ~str) -> bool;
}

struct MemoryBackend {
  reference: HashMap<~str, ~str>
}

impl Backend for MemoryBackend {
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

/// Tests, verifying proper operation of the backend implementations.

#[test]
fn backend_test() {
  let mut backend: MemoryBackend = Backend::new();

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

/// Backend server implementation.  Provides a listening interface for
/// inbound messages, which are converted to messages to the backend
/// itself.

trait BackendServer {
  fn new() -> Self;
}

struct MemoryBackendServer {
  backend: MemoryBackend
}

impl BackendServer for MemoryBackendServer {
  fn new() -> MemoryBackendServer {

    // Open a new TCP Socket.
    let mut acceptor = TcpListener::bind(SocketAddr {
      ip: Ipv4Addr(127, 0, 0, 1), port: 8080
    }).listen().unwrap();

    println(format!("Acceptor is listening on port {:d}", 8080));

    // Spawn a task for every incoming TCP connection.
    loop {

      // Open socket.
      let stream = RefCell::new(acceptor.accept().unwrap());

      // Upon each connection, spawn task for the TCP connection.
      do spawn {

        // Start up a shared channel for the children to talk to the
        // TCP request.
        let (port, chan) = SharedChan::new();

        // Spawn a task to do some work.
        do spawn {

          // First, clone the shared TCP channel for talking to the
          // socket.
          let mut chan = chan.clone();

          // Send a message back to the TCP socket task.
          chan.send(~"done in task");

        }

        // Unwrap the data from the stream, and clone.
        let mut stream = stream.unwrap();

        // TCP connection then blocks on the three responses.
        let response = port.recv();

        // Return to client.
        stream.write(bytes!("OK\r\n"));
      }
    }
  }
}

/// Tests for the memory based backend TCP server.

#[test]
fn backend_server_test() {
  let mut backend_server: MemoryBackendServer = BackendServer::new();
}

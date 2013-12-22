#[desc = "Backend server implementation for Pyrite"];
#[license = "MIT"];

mod backend_server {

  use backend::MemoryBackend;

  use std::cell::Cell;
  use std::rt::io::{Writer, Listener, Acceptor};
  use std::rt::io::net::tcp::TcpListener;
  use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};

  #[test]
  fn basic_test() {
    let mut acceptor = TcpListener::bind(SocketAddr {
      ip: Ipv4Addr(127, 0, 0, 1), port: 8080
    }).listen().unwrap();

    println(format!("Acceptor is listening on port {:d}", 8080));

    loop {
      let stream = Cell::new(acceptor.accept().unwrap());

      do spawn {
        let mut stream = stream.take();
        stream.write(bytes!("Hello World!\r\n"));
      }
    }
  }

}

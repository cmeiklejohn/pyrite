#[desc = "The Pyrite Rust package."];
#[license = "MIT"];

extern mod extra;

use extra::comm::DuplexStream;

fn main() {
  let (server, client) = DuplexStream();

  do spawn {
    let mut buffer: ~str;

    loop {
      buffer = server.recv();
      println(format!("Closure received: {:s}", buffer));
      server.send(buffer);
    }
  }

  do 5.times {
    client.send(~"Hello");
    println(format!("Server received back: {:s}", client.recv()));
  }
}

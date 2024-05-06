pub fn req_status() -> impl Iterator<Item = u32> {
  0..1000
}

pub fn check_status(status: u32) -> () {
  match status {
      100 => println!("Continue"),
      101 => println!("Switching Protocols"),
      200 => println!("OK"),
      201 => println!("Created"),
      202 => println!("Accepted"),
      204 => println!("No Content"),
      301 => println!("Moved Permanently"),
      302 => println!("Found"),
      304 => println!("Not Modified"),
      400 => println!("Bad Request"),
      401 => println!("Unauthorized"),
      403 => println!("Forbidden"),
      404 => println!("Not Found"),
      405 => println!("Method Not Allowed"),
      408 => println!("Request Timeout"),
      418 => println!("I'm a teapot"),
      429 => println!("Too Many Requests"),
      500 => println!("Internal Server Error"),
      501 => println!("Not Implemented"),
      502 => println!("Bad Gateway"),
      503 => println!("Service Unavailable"),
      504 => println!("Gateway Timeout"),
      other => {
          println!("Unknown status code: {}", other);
          // get response from cache
      }
}
}
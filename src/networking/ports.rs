use std::io;

pub fn read_port() -> Result<u16, String> {
    println!("\nEnter port number: ");
    let mut buf = String::new();
    if let Err(r) = io::stdin().read_line(&mut buf) {
        return Err(r.to_string());
    }

    buf.pop(); // Remove '\n' from stdin
    
    match buf.parse::<u16>() {
        Ok(u) => Ok(u),
        Err(r) => Err(r.to_string())
    }   
}
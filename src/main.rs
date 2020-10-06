use std::fmt;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::io::{Write, Error, BufReader, BufRead};

//use std::result::Result;

fn getline(stream: &TcpStream) -> String {

	let mut reader = BufReader::new(stream);
	let mut line = String::new();

	while !line.ends_with("\r\n"){
		let len = reader.read_line(&mut line)
		                .expect("Wrong!");
		reader.consume(len);

		if len == 0 {
			break;
		}

//		println!("{}", line);
	}

	return line;

}

struct Entry {
	item_type: String,
	display_string: String,
	selector: String,
	hostname: String,
	port: u16
}

impl fmt::Display for Entry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
		write!(f, "{}{}\t{}\t{}\t{}\r\n", self.item_type, self.display_string,
		                              self.selector, self.hostname,
		                              self.port)
	}
}

static mut entries: Vec<Entry> = vec![];

fn search_entries(who: String) -> Result<&'static Entry, &'static u8> {
	unsafe {
		// TODO: if using threads mutex this shit
		for entry in & entries {
			let mut s = entry.selector.clone();
			s.push_str("\r\n");
			if who == s {
				return Ok(entry);
			}
		}
	}
	return Err(&0);
}
/*
fn send_list(stream: & TcpStream) {
	// update_list()
	let list = fs::read_to_string("gopher.db");
	match list {
		// TODO: comply with final .
		Ok(s) => stream.write(s.as_bytes()),
		Err(s) => stream.write("Error retrieving file list".as_bytes()),
	};
}
*/
fn answer(mut stream: &TcpStream, file: String){
	// TODO: update_list();
	let f = fs::read_to_string(file);
	// TODO: security consideration: check file is in database
	match f {
		Ok(s) => stream.write(s.as_bytes()),
		Err(_) => stream.write("Error retrieving file.".as_bytes()),
	};
	stream.write("\r\n.\r\n".as_bytes());
}

fn handle_client(stream: &TcpStream){
	let received = getline(& stream);
	println!("received: {}", received);

	//later it will get populated each time an entry in the filesystem is updated
	let entry = search_entries(received);

	match entry {
		Ok(e) => answer(& stream, e.selector.clone()),
		Err(_) => answer(& stream, "gopher.db".to_string()),
	};
}




fn main() -> Result<(), Error> {
	let listener = TcpListener::bind("127.0.0.1:3000")?;
	let port = listener.local_addr()?;
	println!("Listening on {}", port);

	for stream in listener.incoming() {
		// let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
		println!("Connection received! sending data."); //, addr);
		handle_client(&mut stream?);
	}
	Ok(())
}

use std::fmt;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::io::{Write, Error, BufReader, BufRead};


mod test;




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

fn search_entries(who: String, entries: & Vec<Entry>) -> Result<& Entry, u8> {
//	unsafe {
		// TODO: if using threads mutex this shit
		for entry in entries {
			let mut s = entry.selector.clone();
			s.push_str("\r\n");
			if who == s {
				return Ok(entry);
			}
		}
//	}
	return Err(0);
}

fn answer(mut stream: &TcpStream, file: String){
	// TODO: update_list();
	let f = fs::read_to_string(file);
	// TODO: security consideration: check file is in database
	match f {
		Ok(s) => stream.write(s.as_bytes()).expect("Couldn't write to socket.\n"),
		Err(_) => stream.write("Error retrieving file.".as_bytes()).expect("Couldn't write to socket.\n"),
	};
	stream.write("\r\n.\r\n".as_bytes()).expect("Couldn't write to socket.\n");
}

fn update_list(entries: & mut Vec<Entry>){

	let entry = Entry {
		item_type: "a".to_string(),
		display_string: "a".to_string(),
		selector: "a".to_string(),
		hostname: "a".to_string(),
		port: 0};

	entries.push(entry);

}

fn handle_client(stream: &TcpStream, mut entries: & mut Vec<Entry>){
	let received = getline(& stream);
	println!("received: {}", received);

	//later it will get populated each time an entry in the filesystem is updated
	let entry = search_entries(received, entries);

	match entry {
		Ok(e) => answer(& stream, e.selector.clone()),
		Err(_) => answer(& stream, "gopher.db".to_string()),
	};

	update_list(& mut entries);

}

fn main() -> Result<(), Error> {
	let listener = TcpListener::bind("127.0.0.1:3000")?;
	let port = listener.local_addr()?;
	println!("Listening on {}", port);

	let mut entries: Vec<Entry> = vec![];

	for stream in listener.incoming() {
		// let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
		println!("Connection received! sending data."); //, addr);
		handle_client(&mut stream?, &mut entries);
	}
	Ok(())
}

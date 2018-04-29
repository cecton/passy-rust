use std::io::Write;

extern crate base64;
extern crate rpassword;
extern crate sha1;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
	let cmd = &args[0];

	let pass = rpassword::prompt_password_stderr("Password: ").unwrap();
	println!();

	eprintln!("Hint: {}", make_password(&pass, "foo").get(0..6).unwrap());

	match args.get(1).map(|x| x.as_ref()) {
		Some("--help") => usage(cmd),
		Some(site) => println!("{}", make_password(&pass, site)),
		None => println!("{}", make_password(&pass, &ask_site().unwrap())),
	}
}

fn usage(cmd: &str) {
	println!("usage: {} [<site>]", cmd);
}

fn ask_site() -> std::io::Result<String> {
	let mut stderr = std::io::stderr();
	write!(stderr, "Site: ")?;
	stderr.flush()?;

	let mut site = String::new();
	match ::std::io::stdin().read_line(&mut site) {
		Ok(_) => Ok(site.chars().take_while(|x| x != &'\n').collect()),
		Err(err) => Err(err),
	}
}

fn make_password(p: &str, s: &str) -> String {
	let digest = sha1::Sha1::from(format!("_{}_{}_", p, s)).digest().bytes();
	let b64 = base64::encode(digest.as_ref());
	return passwordify(&b64);
}

fn passwordify(s: &str) -> String {
	let bytes = s.as_bytes();
	let symbols: &[u8] = "!?+-=*/@#$%&()[];:,.<>".as_bytes();

	return format!("{}{}{}{}{}",
		("A".as_bytes()[0] + bytes[0] % 26) as char,
		("a".as_bytes()[0] + bytes[1] % 26) as char,
		("0".as_bytes()[0] + bytes[2] % 10) as char,
		symbols[bytes[3] as usize % symbols.len()] as char,
		s.get(4..26).unwrap());
}

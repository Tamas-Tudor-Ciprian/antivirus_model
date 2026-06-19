use std::fs;



fn main() {

	let contents = fs::read_to_string("file.pdf").expect("Failed to read file");


	println!("These are the raw contents of the file: \n{}", contents);


	//I'll just check if its a pdf initially and then I will setup the api

	
	let magic = &contents[..4];


	if magic == "%PDF"{

		println!("Yes this is a pdf file get him!");



	}
	else {

		println!("Safe");

	}
	
	
	
}

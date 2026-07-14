



fn detect_file(path: &str) -> String{

	let mut to_return = String::new();

	let contents = fs::read_to_string(path).expect("Idk what you gave me I can't find the file mate...\n");



	println!("These are the raw contents of the file if you wanna check: \n{}", contents);


	//I'll be using the structs from file types to generate file specific output , keeping interfaces dynamic



	let magic = &contents[..8];


	if magic == "%PDF"{
		
		to_return = "Pdf file here".to_string();


	

	 }
	else {

		to_return = "whaterver you wanna output".to_string();
	

		}


	to_return




}

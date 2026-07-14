
//this should cover a couple common file types
const MAGIC_PNG:u64 = 0x89_50_4E_47_0D_0A_1A_0A;
const MAGIC_PDF:u64 = 0x25_50_44_46_2D;
const MAGIC_JPEG:u64 = 0x_FF_D8_FF;
const MAGIC_GIF78A:u64 = 0x47_49_38_37_61;
const MAGIC_GIF89A:u64 = 0x47_49_46_38_39_61;
const MAGIC_ZIP:u64 = 0x_50_4B_03_04;
const MAGIC_ELF:u64 = 0x7F_45_4C_46;
const MAGIX_EXE:u64 = 0x_4D_5A;
const MAGIC_JAVA:u64 = 0x_CA_FE_BA_BE;

const FILES_NR:u64 = 9;

struct Files{
	magic : u64,
	name: String,
	extension: String,
}

//on a second thought this should all be stored in a json

impl Files {

	pub fn define(magic: u64, name: &str, extension: &str) ->Self{

		Self{
			magic: magic,
			name: name.to_string(),
			extension: extension.to_string(),
		}
	}

	pub fn get_types() ->Vec<Files>{
		
		let mut file_types = Vec::<Files>::new();


		//we init all the structs here
		let png = Files::define(MAGIC_PNG, "png", ".png");
		file_types.push(png);
		let pdf = Files::define(MAGIC_PDF, "pdf", ".png");
		file_types.push(pdf);
		let jpeg = Files::define(MAGIC_JPEG,"jpeg", ".jpg");
		file_types.push(jpeg);
		let gif87a = Files::define(MAGIC_GIF78A,"gif78a", ".gif");
		file_types.push(gif87a);
		let gif89a = Files::define(MAGIC_GIF89A,"gif89a", ".gif");
		file_types.push(gif89a);
		let zip = Files::define(
	
		file_types

	}

}

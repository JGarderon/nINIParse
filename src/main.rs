
mod lib; 

fn main() {
	match lib::INIContext::new( "./test.ini", true ) { 
		Ok( tree ) => { 
			println!( 
				"{:?} ou {:?}", 
				tree.search( "generaly".to_string(), "name".to_string() ), 
				tree.search( "generaly".to_string(), "rien".to_string() ) 
			); 
		}, 
		Err( err ) => println!( "err : {:?}", err ) 
	} 
} 

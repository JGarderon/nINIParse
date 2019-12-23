
// --------------------------------------------------------------------
// --------------------------------------------------------------------
// - 
// ! Author / Auteur : Julien Garderon 
// - 
// - Date : 22 décembre 2019 
// - Version : 1.0 - Parseur de fichier INI ("INI file Parser") 
// - Name / nom GITHUB : nINIParse 
// - 
// ! Description : 
// -  
// - 	INITContext est une énumération qui peut lire un fichier 
// - 	ou une chaine de caractères (String), et ressortir un 
// - 	INITTree. Cette structure implémente la fonction de recherche 
// - 	"search", qui retourne une option (la valeur ou "None"). 
// - 	A noter que la section "generaly" est toujours automatiquement
// - 	ajoutée à INITTree et correspond à la section par défaut. 
// -  
// ! Dependencies / Dépendances : none / aucune 
// -  
// --------------------------------------------------------------------
// --------------------------------------------------------------------

use std::fs::File; 
use std::io::Read; 

#[derive(Debug)] 
pub enum INIContext { 
	None, 
	Key, 
	Value, 
	Line, 
	Comment, 
	Section 
} 

impl INIContext { 
	pub fn new( chemin: &str, strict: bool ) -> Result<INITree, String> { 
		let f = File::open( chemin ); 
		match f { 
			Ok( mut reader ) => { 
				let mut buffer = String::new(); 
				// ! \ Attention, tout le fichier est chargé en mémoire 
				if let Ok(_) = reader.read_to_string( &mut buffer ) { 
					match INIContext::parse_string( &mut buffer, strict ) { 
						Result::Ok( arbre ) => return Result::Ok( arbre ), 
						Result::Err( err ) => return Result::Err( format!( 
							"le fichier n'est pas correct, position : {:?}", 
							err 
						) ) 
					} 
				} else { 
					return Result::Err( "Oups ! le fichier ne peut pas être lu".to_string() ); 
				} 
			}, 
			Err( err ) => return Result::Err( format!( 
				"Oups ! le fichier ne peut pas être ouvert : {:?}", 
				err 
			) ) 
		} 
		
	}
	pub fn parse_string( buffer: &mut String, strict: bool ) -> Result<INITree, usize> { 
		let mut contexte: INIContext = INIContext::None; 
		let mut arbre: INITree = INITree { 
			leaves: Vec::new() 
		}; 
		arbre.leaves.push( 
			( INIContext::Section, "generaly".to_string() ) 
		); 
		let mut portion: Vec<char> = Vec::new(); 
		for (i, c) in buffer.chars().enumerate() { 
			match contexte { 
				INIContext::None => { 
					match c { 
						';' => contexte = INIContext::Comment, 
						'[' => contexte = INIContext::Section, 
						'\r' | '\n' => (), 
						'a'..='z' | 'A'..='Z' | '0'..='9' => { 
							portion.push( c ); 
							contexte = INIContext::Key; 
						}, 
						_ => if strict { return std::result::Result::Err( i ); } 
					} 
				} 
				INIContext::Key => { 
					match c { 
						'=' => { 
							let s: String = portion.iter().collect(); 
							arbre.leaves.push( 
								( INIContext::Key, s ) 
							); 
							portion.clear(); 
							contexte = INIContext::Value; 
						} 
						'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' | '~' => portion.push( c ), 
						_ => if strict { return std::result::Result::Err( i ); } 
					} 
				} 
				INIContext::Value => { 
					match c { 
						'\n' | '\r' => { 
							let s: String = portion.iter().collect(); 
							arbre.leaves.push( 
								( INIContext::Value, s ) 
							); 
							portion.clear(); 
							if c=='\r' { 
								contexte = INIContext::Line; 
							} else { 
								contexte = INIContext::None; 
							} 
						} 
						_ => portion.push( c ) 
					} 
				} 
				INIContext::Line => { 
					match c { 
						'\r' => (), 
						'\n' => contexte = INIContext::None, 
						_ => if strict { return std::result::Result::Err( i ); } 
					} 
				} 
				INIContext::Section => { 
					match c { 
						']' => { 
							let s: String = portion.iter().collect(); 
							arbre.leaves.push( 
								( INIContext::Section, s ) 
							); 
							portion.clear(); 
							contexte = INIContext::Line; 
						} 
						_ => portion.push( c ) 
					} 
				} 
				INIContext::Comment => { 
					match c { 
						'\n' | '\r' => { 
							let s: String = portion.iter().collect(); 
							arbre.leaves.push( 
								( INIContext::Comment, s ) 
							); 
							portion.clear(); 
							if c=='\r' { 
								contexte = INIContext::Line; 
							} else { 
								contexte = INIContext::None; 
							} 
						} 
						_ => portion.push( c ) 
					} 
				}
			} 
		} 
		std::result::Result::Ok( arbre ) 
	} 
} 

#[derive(Debug)] 
pub struct INITree { 
	leaves: Vec<(INIContext, String)>
} 

impl INITree { 
	pub fn search( &self, section: String, cle: String ) -> Option<String> { 
		let mut c1: bool = false; 
		let mut c2: bool = false; 
		for feuille in &self.leaves { 
			let (contexte, chaine) = feuille; 
			match contexte {
				INIContext::Section => if *chaine==section { 
					c1 = true; 
				} 
				INIContext::Key => if *chaine==cle { 
					c2 = true; 
				} 
				INIContext::Value => if c1 && c2 { 
					return Some( chaine.clone() ) 
				} 
				_ => () 
			} 
		} 
		None 
	} 
} 


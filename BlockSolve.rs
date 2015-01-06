use std::io::File;
use std::io::BufferedReader;

use std::collections::HashSet;
use std::collections::HashMap;

fn main(){
	let mut unique_sets = HashSet::new();
	let mut main_map = HashMap::new();
	load_input(&mut main_map);
	
	for a in main_map.iter(){	
		println!("Origial sequence {}", a );	
	}
	
	let mut a_count: uint = 0;	
	let mut block_count = 0u;
	for (a_k, a_v) in main_map.iter(){
		unique_sets.insert(a_k);
		a_count += 1;
		//println!("\n A: {}, {}", a_k , a_v );
			
		for (b_k, b_v) in main_map.iter().skip(a_count){						
			//println!(" B: {}, {}", b_k , b_v );
			
			//Intersect the two protein sets
		/* 	for x in a_v.intersection(b_v) {
				println!("{}", x);			
			} */
			let intersection : HashSet<uint> = a_v.intersection(b_v).map(|&x| x).collect();
			if intersection.len() > 1{
				println!("{} {} {}", a_k, b_k, intersection );
				block_count +=1;
			}						
		}
	}

	println!("\n Number of blocks {}", block_count );
	
	println!("Number of Unique Blocks {}", unique_sets.len() );
	
}

fn load_input(main_map: &mut HashMap<String,HashSet<uint>>){
	// Create a path to the desired file
    let path = Path::new("random_input.txt");
	
	let mut file = BufferedReader::new(File::open(&path));
	for line in file.lines().filter_map(|result| result.ok()) {		
		let protein;
		let mut c_set = HashSet::new();
			
		//Find the index of the first comma to get protein		
		let first_comma = line.find(',');		
		match first_comma {
			Some(x) =>{
				//Split the line from index 0 to the first comma to get the protein value				
				protein = line.slice(0,x);								
			}
			None => panic!("Did not find a protein")			
		}				
		
		let split_line = line.split_str(",").map(|s| s.trim());
		let mut split_line_vec: Vec<&str> = split_line.collect();
		split_line_vec.remove(0);
				
		for i in split_line_vec.iter(){	
			let c_ref: uint = from_str(i.as_slice()).unwrap();
	/* 		let c_ref :uint = match from_str(i.as_slice()){				
				Some(x) => x,
				None =>  break;,
			};  */
			//println!("{}", c_ref );			
			c_set.insert(c_ref);
		}		
		
		main_map.insert(protein.to_string(),c_set);	
	}
		
}
 
fn create_p_map(){
	let mut p_map = HashMap::new();
	
 	let mut num_protein_sets = 5000i;
	while num_protein_sets > 0{	
		let mut new_set: HashSet<int> = HashSet::new();	
		
		for n in range(1i,10000){
			new_set.insert(n);
			//if n%1000000 == 0{println!("{}",n);}			 
		}
		p_map.insert(num_protein_sets,new_set);		
		println!("{}",num_protein_sets);
		num_protein_sets = num_protein_sets -1 ;
	}
}


fn infinite(){
	loop {	
	}
}

use std::io::File;
use std::io::BufferedReader;

use std::collections::HashSet;
use std::collections::HashMap;

fn main(){
	let mut solutions_map: HashMap<String,Vec<Solutions_struc>> = HashMap::new();

	struct Solutions_struc{
		a: String,
		b: String,
		set_length: uint,
		set: HashSet<uint>
	}
	
	let mut all_sub_solutions: Vec<Solutions_struc> = vec![];

	let mut unique_sets: Vec<&HashSet<uint>> = vec![];
	let mut main_map = HashMap::new();
	load_input(&mut main_map);
	
	for a in main_map.iter(){	
		println!("Origial sequence {}", a );	
	}
	
	let mut a_count: uint = 0;		
	for (a_k, a_v) in main_map.iter(){		
		a_count += 1;
		//println!("\n A: {}, {}", a_k , a_v );
		let mut all_solutions: Vec<Solutions_struc> = vec![];
	
		for (b_k, b_v) in main_map.iter().skip(a_count){						
			//println!(" B: {}, {}", b_k , b_v );
			
			//Intersect the two protein sets
		/* 	for x in a_v.intersection(b_v) {
				println!("{}", x);			
			} */
			let intersection : HashSet<uint> = a_v.intersection(b_v).map(|&x| x).collect();
			if intersection.len() > 1{
				println!("{} {} {}", a_k, b_k, intersection );				
				//all_sets.push(intersection);				
				let solution = Solutions_struc{ a: a_k.to_string(), b: b_k.to_string(), set_length: intersection.len(), set: intersection };
				all_solutions.push(solution);
			}						
		}
		
		solutions_map.insert(a_k.to_string(),all_solutions);		
	}
	
	for (k,v) in solutions_map.iter(){
		for a in v.iter(){
			println!("Found solution {} {} {}", a.a, a.b, a.set );	
			
			a_count += 1;
			for b in v.iter().skip(a_count){
				if a.set.is_subset(&b.set){
					let solution = Solutions_struc{ a: a.a.to_string(), b: b.b.to_string(), set_length: a.set_length, set: a.set.clone() };
					all_sub_solutions.push(solution);
					
					println!("Found sub solution {} {} {}", a.a, b.b, a.set );	
				}	
			}
		}	
	}
	
/* 	for (k,v) in solutions_map.iter(){
		for a in v.iter(){
			println!("Found sub solution {} {} {}", a.a, a.b, a.set );			
		}	
	}
	
	for n in all_sub_solutions.iter(){
		println!("Found sub solution {} {} {}", n.a, n.b, n.set );
	} */
	
	
		
	
		
/*  	let mut duplicate = HashSet::new();
	duplicate.insert(25);
	duplicate.insert(39);
	duplicate.insert(30);
	duplicate.insert(47);	
	let mut solution = Solutions_struc{ a: "Dummy A".to_string(), b: "Dummy B".to_string(), set_length: duplicate.len(), set: duplicate };
	all_solutions.push(solution);
	
	duplicate = HashSet::new();
	duplicate.insert(19);
	duplicate.insert(18);
	duplicate.insert(33);
	duplicate.insert(4);	
	duplicate.insert(6);	
	solution = Solutions_struc{ a: "Dummy A".to_string(), b: "Dummy B".to_string(), set_length: duplicate.len(), set: duplicate };
	all_solutions.push(solution); */

	
	//println!("\nNumber of blocks {}", all_solutions.len() );	
		
	//Get unique blocks
/* 	let mut dup = false;
	let mut set_count = 0u;
	a_count = 0;
	for a in all_solutions.iter(){	
		a_count += 1;	
		set_count = 0;
		dup = false;
		for b in all_solutions.iter().skip(a_count){			
			if a.set == b.set{
				println!("Found Duplicate {} at index {}", b.set, set_count + a_count );
				dup = true;	
				break;
			}				
			set_count += 1;			
		}
		
		if	dup == false{
			unique_sets.push(&a.set);
		}
	}	
 	println!("\nNumber of Unique Blocks {}", unique_sets.len() ); */
	
/* 	for n in unique_sets.iter(){
		println!("Unique Block {}", n );	
	}  */
	
		
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
			let c_ref: uint = i.as_slice().parse::<uint>().unwrap();
	/* 		let c_ref :u32 = match from_str(i.as_slice()){				
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

use std::io::File;
use std::io::BufferedReader;

use std::collections::HashSet;
use std::collections::HashMap;

struct SolutionsStruc{
	protein: String,		
	length: uint,
	set: HashSet<uint>
}
	
fn main(){
	let mut solutions_map: HashMap<String,Vec<SolutionsStruc>> = HashMap::new();

	let mut all_sub_solutions: Vec<SolutionsStruc> = vec![];
	
	let mut main_map = HashMap::new();
	load_input(&mut main_map);
	
	for a in main_map.iter(){	
		println!("Original sequence {}", a );	
	}
	
	let mut a_count: uint = 0;		
	for (a_k, a_v) in main_map.iter(){		
		a_count += 1;
		//println!("\n A: {}, {}", a_k , a_v );
		let mut all_solutions: Vec<SolutionsStruc> = vec![];
		
		println!("\n");
	
		for (b_k, b_v) in main_map.iter().skip(a_count){						
			//println!(" B: {}, {}", b_k , b_v );
			
			//Intersect the two protein sets
		/* 	for x in a_v.intersection(b_v) {
				println!("{}", x);			
			} */
			let intersection : HashSet<uint> = a_v.intersection(b_v).map(|&x| x).collect();
			if intersection.len() > 1{
				//println!("{} {} {}", a_k, b_k, intersection );
				
				println!("Found solution {} {}", a_k, intersection );
				println!(" Found solution {} {}", b_k, intersection );
				
				insert_solution(a_k,&intersection,&mut all_solutions);
				insert_solution(b_k,&intersection,&mut all_solutions);
				
				//let mut solution = SolutionsStruc{ protein: a_k.to_string(), length: intersection.len(), set: intersection };
				//all_solutions.push(solution);				
				
				//solution = SolutionsStruc{ protein: b_k.to_string(), length: intersection.len(), set: intersection };				
				//all_solutions.push(solution);
			}						
		}
		
		//Find sub solutions
		find_sub_solutions(&mut all_solutions);
		
		solutions_map.insert(a_k.to_string(),all_solutions);	
/* 
		println!("\n");
		for (k,v) in solutions_map.iter(){
			for n in v.iter(){
				println!("Found solution {} {}", k, n.set );	
			}
		} */
		
	}
	
/* 	for (k,v) in solutions_map.iter(){
		println!("\n{}", k );
		for a in v.iter(){
			println!("Found solution {} {} {}", a.a, a.b, a.set );	
			
			a_count += 1;
			for b in v.iter(){
				if a.set == b.set || a.length > b.length{
					continue;
				}
								
				if a.set.is_subset(&b.set){
					let solution = SolutionsStruc{ a: a.a.to_string(), b: b.b.to_string(), length: a.length, set: a.set.clone() };
					all_sub_solutions.push(solution);
					
					println!("Found sub solution {} {} {}", a.a, b.b, a.set );	
				}	
			}
		}	
	} */
	
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
	let mut solution = SolutionsStruc{ a: "Dummy A".to_string(), b: "Dummy B".to_string(), length: duplicate.len(), set: duplicate };
	all_solutions.push(solution);
	
	duplicate = HashSet::new();
	duplicate.insert(19);
	duplicate.insert(18);
	duplicate.insert(33);
	duplicate.insert(4);	
	duplicate.insert(6);	
	solution = SolutionsStruc{ a: "Dummy A".to_string(), b: "Dummy B".to_string(), length: duplicate.len(), set: duplicate };
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

fn insert_solution(protein: &String, set: &HashSet<uint>, all_solutions: &mut Vec<SolutionsStruc>){	
	let solution = SolutionsStruc{ protein: protein.to_string(), length: set.len(), set: set.clone() };
	all_solutions.push(solution);	
}

fn contains_solution(all_solutions: &Vec<SolutionsStruc>, solution: &SolutionsStruc) -> bool{
	for n in all_solutions.iter(){
		if n.set == solution.set{
			return true;
		}
	}
	return false;
}
fn find_sub_solutions(all_solutions: &mut Vec<SolutionsStruc>){
	let mut all_sub_solutions: Vec<SolutionsStruc> = vec![];

	for a in all_solutions.iter(){
		//println!("Found solution {} {} {}", a.a, a.b, a.set );	
	
		for b in all_solutions.iter(){
			//Only attempt finding a subset if the length of "a" is less than than my length "b" and it is not me
			if a.set == b.set || a.length >= b.length || contains_solution(&all_sub_solutions,a){
				continue;
			}					
							
			if a.set.is_subset(&b.set){
				let solution = SolutionsStruc{ protein: b.protein.to_string(), length: a.length, set: a.set.clone() };
				all_sub_solutions.push(solution);
				
				println!("Found sub solution {} {}", b.protein, a.set);					
			}	
		}
	}
	
	//Add the subsets into the main set
	for n in all_sub_solutions.drain(){
		all_solutions.push(n);
	}
	
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

use std::io::File;
use std::io::BufferedReader;

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Show)]
struct PMap {
    protein: String,
    set_ref: uint,
}
	
fn main(){
	let mut all_sets: HashMap<uint,HashSet<uint>> = HashMap::new();	
	let mut p_map: HashSet<PMap> = HashSet::new();		
	
	let mut main_map = HashMap::new();
	load_input(&mut main_map);
	
	
/* 	for a in main_map.iter(){	
		println!("Original sequence {}", a );	
	} */
	
	let mut a_count: uint = 0;		
	for (a_k, a_v) in main_map.iter(){		
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
				//println!("{} {} {}", a_k, b_k, intersection );
				insert_p_map(a_k, &intersection, &mut p_map,&mut all_sets);	
				insert_p_map(b_k, &intersection, &mut p_map,&mut all_sets);	
			}						
		}						
	}
	
	find_sub_sets(&all_sets,&mut p_map);
	
	for n in p_map.iter(){
		println!("P Map {} {}", n.protein, n.set_ref );
	}
	
	for (k,v) in all_sets.iter(){
		println!("All Set {} {}", k, v );	
	}			
	
	//Print how many of each set there are
	let mut unique_set_count: HashSet<uint> = HashSet::new();
	for a in p_map.iter(){
		let mut count = 0u;
		if unique_set_count.contains(&a.set_ref){
			continue;
		}
		unique_set_count.insert(a.set_ref);
		
		let mut proteins: HashSet<String> = HashSet::new();
		for b in p_map.iter(){
			if a.set_ref == b.set_ref{
				count+=1;
				proteins.insert(b.protein.to_string());
			}
		}		
		println!("Set {} Count {} Proteins {}", a.set_ref ,count, proteins);
	}
	
}

 //Insert or Update master list of sets and return the reference number to the set
 fn modify_set_ref(set: &HashSet<uint>, all_sets: &mut HashMap<uint,HashSet<uint>>) -> uint{
	let ref_num;
	
	for (k,v) in all_sets.iter(){
		if *v == *set{			
			return *k;
		}
	}
	
	ref_num = all_sets.len() + 1;	
	all_sets.insert(ref_num,set.clone());	
	return ref_num;	
} 

fn insert_p_map(protein: &String, set: &HashSet<uint>, p_map: &mut HashSet<PMap>, all_sets: &mut HashMap<uint,HashSet<uint>>){
	let set_ref = modify_set_ref(set,all_sets);
		
	p_map.insert(PMap { protein: protein.to_string(), set_ref: set_ref });		
}

fn find_sub_sets(all_sets: &HashMap<uint,HashSet<uint>>, p_map: &mut HashSet<PMap>){
	for (a_k,a_v) in all_sets.iter(){
		let a_len = a_v.len();	
		
		for (b_k,b_v) in all_sets.iter(){
			if a_len < b_v.len(){
				if a_v.is_subset(b_v){
					add_sub_set(a_k,b_k,p_map);						
				}				
			}			
		}		
	}	
}

fn add_sub_set(subset: &uint, superset: &uint, p_map: &mut HashSet<PMap>){
	let mut tmp_p_map: HashSet<PMap> = HashSet::new();

	//Find subsets within the original sets that were found for this protein
	for n in p_map.iter(){
		if n.set_ref == *superset{
			//Add to p_map with subset ref as well
			//println!("Found subset protein:{} subset_ref:{} superset:{}", n.protein, subset,superset);
			tmp_p_map.insert(PMap { protein: n.protein.to_string(), set_ref: *subset });
		}
	}
	
	for n in tmp_p_map.drain(){
		p_map.insert(n);
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

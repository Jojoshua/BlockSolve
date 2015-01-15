//extern crate time;

use std::io::File;
use std::io::BufferedReader;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Arc;

#[derive(Hash, Eq, PartialEq, Show)]
struct PMap {
    protein: String,
    set_ref: usize,
}

struct IntersectionResult{
    protein_a: String,
	protein_b: String,
    intersection: Vec<usize>,
}


/* struct Timer<'a> {
    name: &'a str,
    start: f64,	
} */

/* impl<'a> Timer<'a> {	
    fn new<'a>(name: &'a str) -> Timer{
		Timer{name: name, start: time::precise_time_s()}
    }
	
	fn stop(&self) -> f64{
		let diff  = time::precise_time_s() - self.start ;	
		println!("{} - {}", self.name , diff );
		diff	
	}
} */
	
fn main(){
	//let begin = time::precise_time_s();
	//let end;
		
	let mut all_sets: HashMap<usize,HashSet<usize>> = HashMap::new();	
	let mut p_map: HashSet<PMap> = HashSet::new();		
	
	let mut test = HashMap::new();
	//let mut test: HashMap<Vec<usize>,HashSet<String>> = HashMap::new();
/* 	test.insert(vec![1,2,3,4,5],"1".to_string());
	test.insert(vec![1,2],1);
	test.insert(vec![4,5],1); */
	
/* 	for (a_k,a_v) in test.iter(){
		let a_set: HashSet<usize> = a_k.clone().into_iter().collect();
		let a_len = a_k.len();
		
		// Iterate through sets that are in smaller length		
		for (b_k,b_v) in test.iter().filter( |&(k,v)| k.len() < a_len ){
			let b_set: HashSet<usize> = b_k.clone().into_iter().collect();
			if b_set.is_subset(&a_set){
				println!("{:?} contains {:?}",a_k,b_k);
			}				 
		} 
	} */
	let test2 = Arc::new(test);
		
		
	let mut main_map = HashMap::new();	
	load_input(&mut main_map);
	
	let shared_main_map = Arc::new(main_map);
		
/*  	for a in main_map.iter(){	
		println!("Original sequence {}", a );	
	}  */
	//let t: Timer = Timer::new("round one");
	let mut a_count: usize = 0;		
	
/*   	for (a_k, a_v) in shared_main_map.iter(){
		println!("{}", a_k);
	}  
	 */
	//println!("\n");
		
  	let num_items = shared_main_map.len();
	
	
/* 	for num in range(0,num_items){		
		do_intersection(test2.clone(),num, shared_main_map.clone(),&mut p_map,&mut all_sets);
	}   */
	
	
	do_intersection(test2.clone(), 0, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 100, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 200, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);	
	do_intersection(test2.clone(), 300, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);  
	do_intersection(test2.clone(), 400, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 500, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 600, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 700, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 800, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 900, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	do_intersection(test2.clone(), 1000, 100 , shared_main_map.clone(),&mut p_map,&mut all_sets);
	infinite();
	
/* 	let num_items = main_map.len();
	for num in range(0,num_items){
		println!("Num {}", num);
		
	  	for (a_k, a_v) in main_map.iter().skip(num).take(1){	
			for (b_k, b_v) in main_map.iter().skip(a_count){
				let intersection : HashSet<usize> = a_v.clone().intersection(b_v).map(|&x| x).collect();
				if intersection.len() > 1{
					//println!("{} {} {}", a_k, b_k, intersection );
					//insert_p_map(a_k, &intersection, &mut p_map,&mut all_sets);	
					//insert_p_map(b_k, &intersection, &mut p_map,&mut all_sets);	
				} 
			}        
		} 
	} */
	
/*   	for (a_k, a_v) in main_map.iter(){
		a_count += 1;
 		println!("Num {}", a_count);	
		for (b_k, b_v) in main_map.iter().skip(a_count){
			let intersection : HashSet<usize> = a_v.clone().intersection(b_v).map(|&x| x).collect();
			if intersection.len() > 1{
				//println!("{} {} {}", a_k, b_k, intersection );
				//insert_p_map(a_k, &intersection, &mut p_map,&mut all_sets);	
				//insert_p_map(b_k, &intersection, &mut p_map,&mut all_sets);	
			} 
		}        
	}  */
	 
	
	
	
	

		
/* 		
	if t.stop() < 0.8 {
		//println!("\n Round One was fast {}", t.stop() );
	/* 	for (a_k, a_v) in main_map.iter(){	
			println!("\n{}, {}", a_k , a_v );	
		} */
	} */
	
	//let t: Timer = Timer::new("round two");
	find_sub_sets(&all_sets,&mut p_map);
	/* if t.stop() < 1.0 {
		//println!("\n Round Two was fast {}", t.stop() );	
	/* 	for (a_k, a_v) in main_map.iter(){	
			println!("\n{}, {}", a_k , a_v );	
		} */
	} */
	
	
/* 	for n in p_map.iter(){
		println!("P Map {} {}", n.protein, n.set_ref );
	}
	
	for (k,v) in all_sets.iter(){
		println!("All Set {} {}", k, v );	
	}	 */		
	
	
	//Print how many of each set there are
	let mut unique_set_count: HashSet<usize> = HashSet::new();
	for a in p_map.iter(){
		let mut count = 0;
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
		//println!("Set {} Count {} Proteins {}", a.set_ref ,count, proteins);
	}
	
	
/* 	end = time::precise_time_s();
	println!("Ran in {} S", end - begin ); */
	
}

 fn do_intersection(test: Arc<HashMap<Vec<usize>,usize>> , start_at: usize, take: usize, main_map: Arc<HashMap<String,HashSet<usize>>>, p_map: &mut HashSet<PMap>, all_sets: &mut HashMap<usize,HashSet<usize>>){	
	//let b_count: usize = start_at + 1;
	//let (tx, rx) = channel();
	
	//println!("Num {}", start_at1);		
		
 	Thread::spawn( move || {	
/* 		test.insert(vec![1,2,3,4,5],1);
		test.insert(vec![1,2],1);
		test.insert(vec![4,5],1);  */
		//infinite();
        let mut results: Vec<IntersectionResult> = vec![]; 
		let mut b_count = start_at + 1;
		let mut count: usize = 0;
			
		for (a_k, a_v) in main_map.iter().skip(start_at).take(take){
			count +=1;
		//println!("\n loop a {}", a_k);		
 			 for (b_k, b_v) in main_map.iter().skip(b_count){
			 //println!("loop b {}",b_k);		
				//println!("loop b {} - loop a key {} - b_count {}  - start_at {}", b_k,a_k,b_count,start_at);	
				
				//println!("{} - {}",a_k,b_k);
				
		 	 	let intersection : HashSet<usize> = a_v.intersection(b_v).map(|&x| x).collect();				
				let mut intersection_vec: Vec<usize> = intersection.into_iter().collect();
				intersection_vec.sort(); // Sort the result in order
				//println!("{:?} - {:?}", intersection, intersection_vec);
				
				if intersection_vec.len() > 1{	
					//println!("{} {} {:?}", a_k, b_k, intersection_vec);
					let i_result: IntersectionResult = IntersectionResult{protein_a: a_k.to_string(), protein_b: b_k.to_string(), intersection: intersection_vec};
					results.push(i_result);			
 					
/*					insert_p_map(a_k, &intersection, p_map, all_sets);	 
					insert_p_map(b_k, &intersection, p_map, all_sets);	*/
				}  
			}
			
			b_count+=1;
			println!("1");
			
		}
		//println!("{}", count);	
		//tx.send(results);	
	});	
	
	
 	
	//let results: Vec<IntersectionResult> = rx.recv().unwrap();	 	
/*     for n in results.iter(){
		println!("{} {} {} {:?}", start_at, n.protein_a, n.protein_b, n.intersection);
	} */
	
	//println!("{:?}", rx.recv().unwrap());

	
} 


 //Insert or Update master list of sets and return the reference number to the set
 fn modify_set_ref(set: &HashSet<usize>, all_sets: &mut HashMap<usize,HashSet<usize>>) -> usize{
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

fn insert_p_map(protein: &String, set: &HashSet<usize>, p_map: &mut HashSet<PMap>, all_sets: &mut HashMap<usize,HashSet<usize>>){
	let set_ref = modify_set_ref(set,all_sets);
		
	p_map.insert(PMap { protein: protein.to_string(), set_ref: set_ref });		
}

fn find_sub_sets(all_sets: &HashMap<usize,HashSet<usize>>, p_map: &mut HashSet<PMap>){
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

fn add_sub_set(subset: &usize, superset: &usize, p_map: &mut HashSet<PMap>){
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


fn load_input(main_map: &mut HashMap<String,HashSet<usize>>){
	// Create a path to the desired file
    let path = Path::new("input.txt");
	

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
			let c_ref: usize = i.as_slice().parse::<usize>().unwrap();
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

fn infinite(){
	loop {	
	}
}

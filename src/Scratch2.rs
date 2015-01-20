//extern crate time;

use std::io::{File};
use std::io::BufferedReader;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::RwLock;
use std::io::Timer;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Hash, Eq, PartialEq, Show)]
struct PMap {
    protein: String,
    set_ref: usize,
}

#[derive(Show)]
struct IntersectionResult{
    protein_a: String,
	protein_b: String,
    intersection: Vec<usize>,
}

struct MyThreads{
	finished_count: usize,
	active_count: usize,
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
	let mut timer = Timer::new().unwrap();

	//let begin = time::precise_time_s();
	//let end;	
	//let mut all_sets: HashMap<usize,HashSet<usize>> = HashMap::new();	
	//let mut p_map: HashSet<PMap> = HashSet::new();		
	
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
		do_intersection(num, shared_main_map.clone(),&mut p_map,&mut all_sets);
	}   */
	let p_map_o: HashMap<Vec<String>,usize> = HashMap::new();
	let p_map =  Arc::new(RwLock::new(p_map_o));		
	
	let set_counter = Arc::new(AtomicUsize::new(0));
	let all_sets_o: HashMap<Vec<usize>,usize> = HashMap::new();
	let all_sets =  Arc::new(RwLock::new(all_sets_o));	
		
	//let num_threads_vec: Vec<bool> = Vec::new();
	let thread_info =  Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));
		
	let mut start_at = 0;
	let mut count = 0;	
	let take = if num_items < 100 { 
				  count = 2;
				  100
				} else {					
					num_items / 100					
				};
				
	if count == 0{
		count = take;
	}	
	
	let thread_limiter = 10;
		
	let periodic = timer.periodic(Duration::milliseconds(500));
	while count > 0{	
		// this loop is only executed once every x seconds	
		loop {
			periodic.recv().unwrap();
			if thread_info.read().unwrap().active_count <= thread_limiter{
				println!("Allow another thread to run - Active count {} Finished Count {} \n", thread_info.read().unwrap().active_count ,thread_info.read().unwrap().finished_count );
				break;
			}			
		}		
		
		do_intersection(thread_info.clone(),set_counter.clone(),all_sets.clone(),p_map.clone(), start_at, take, shared_main_map.clone());
		
		start_at = start_at + take;			
		count-=1;
	}
		
	println!("Round One Done" );
	
/*  	let roune_one_results = intersection_map.read().unwrap();
	for n in roune_one_results.iter(){
		println!("{:?}", n );
	} */  
	
	let roune_one_sets = all_sets.read().unwrap();
	for n in roune_one_sets.iter(){
		println!("{:?}", n );
	}
	
	let roune_one_map = p_map.read().unwrap();
	for n in roune_one_map.iter(){
		println!("{:?}", n );
	}
		
	infinite();	

	//find_sub_sets(&all_sets,&mut p_map);
	
/* 	//Print how many of each set there are
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
	} */
	
	
/* 	end = time::precise_time_s();
	println!("Ran in {} S", end - begin ); */
	
}

 fn do_intersection(thread_info: Arc<RwLock<MyThreads>>,set_counter: Arc<AtomicUsize>, all_sets: Arc<RwLock<HashMap<Vec<usize>,usize>>>, p_map: Arc<RwLock<HashMap<Vec<String>,usize>>>, start_at: usize, take: usize, main_map: Arc<HashMap<String,HashSet<usize>>>){	
	Thread::spawn( move || {	
		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count +=1;
			//println!("Begin Thread - Started at {} take {} \n", start_at , take,);			
		}
		
		// Each thread collects its own map until it is finished
		//let mut local_map: HashMap<Vec<usize>,HashSet<String>> = HashMap::new();
		
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
				
				if intersection.len() > 1{	
					let mut intersection_vec: Vec<usize> = intersection.into_iter().collect();
					intersection_vec.sort(); // Sort the result in order
														
					let mut set = all_sets.write().unwrap();						
					if	!set.contains_key(&intersection_vec){
						let next_num = set_counter.fetch_add(1,Ordering::Relaxed);											
						set.insert(intersection_vec.clone(),next_num);												
					}
					match set.get(&intersection_vec.clone()){
						Some(x) => {
/* 		 					let mut new_pset = Vec::new();
							new_pset.push(a_k.to_string());
							new_pset.push(b_k.to_string());
							
							let mut p_set = p_map.write().unwrap();																	
							p_set.insert(new_pset,*x); 	 */
						},
						None => {},					
					}				
									
									
					//set.insert(intersection_vec, num + 1);
								
/* 					let mut new_set = HashSet::new();
					new_set.insert(a_k.to_string());
					new_set.insert(b_k.to_string());
					local_map.insert(intersection_vec.clone(),new_set);	 */
					//println!("{:?}-{}-{}", intersection_vec.clone(), a_k.to_string(),b_k.to_string()  );
					
							
		/* 			let mut protein_set = HashSet::new();
					protein_set.insert(a_k.to_string());
					protein_set.insert(b_k.to_string());					
					
					local_intersections.insert(intersection_vec,protein_set); */
					
					//println!("{:?} - {:?}", intersection, intersection_vec);
					
					// Determine if we already have this set.
					// If we don't add it, else add the proteins to the value set
/* 					{
						let mut set_exists = false;									
						match local_map.get_mut(&intersection_vec) {
							Some(x) => {	
								set_exists = true;	
 								let mut existing_set = x.clone();	
								existing_set.insert(a_k.to_string());
								existing_set.insert(b_k.to_string());
								*x = existing_set;	
								println!("Updating set {:?}", intersection_vec.clone()  );	 							
							},
							None => {
								set_exists = false;		
							},
						}
						
	 					if !set_exists{
							let mut new_set = HashSet::new();
							new_set.insert(a_k.to_string());
							new_set.insert(b_k.to_string());
							local_map.insert(intersection_vec.clone(),new_set);	
							//println!("New set {:?}", intersection_vec.clone()  );
						} 												
					} */		
				}  
			}			
			b_count+=1;				
		}
		
/* 	    // Move local results into global results
		println!("Moving to global" );
		{
			let mut global_map = intersection_map.write().unwrap();
			for (l_k,l_v) in local_map.iter(){		
				let mut set_exists = false;	
				
				match global_map.get_mut(l_k) {
					Some(x) => {	
						set_exists = true;						
						let mut existing_set = x.clone();					
						for n in l_v.iter(){
							existing_set.insert(n.to_string());						
						}	
						*x = existing_set;						bl			
					},
					None => {
						set_exists = false;		
					},
				}
				
				if !set_exists{				
					global_map.insert(l_k.clone(),l_v.clone());	
				}
			} 
		} */
		

		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count -=1;
			thread.finished_count +=1;
			//println!("End Thread - Started at {} take {} active threads {} \n", start_at , take, thread.active_count);
		}
	});	
	
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

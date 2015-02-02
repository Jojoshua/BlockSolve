//extern crate time;
extern crate xxhash;
use std::old_io::{File, Open, ReadWrite};
use std::old_io::BufferedReader;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::old_io::Timer;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use xxhash::{hash};

struct MyThreads{
	finished_count: usize,
	active_count: usize,
}

#[derive (Show,Clone)]
struct Config{
	max_threads: usize,
	split_by_subs: usize,
	round_one_wait_interval: i64,
	input_path: String,		
	print_blocks:bool,
	conserve_memory_at_cost_of_speed:bool,
}

impl Config {	
    fn new() -> Config{		
		let path = Path::new("../config");		
		let display = path.display();
	
		let ofile = match File::open_mode(&path, Open, ReadWrite) {
			Ok(f) => f,
			Err(e) => panic!("config file error: {} {}", display,e.desc),
		};
		
		let mut file = BufferedReader::new(ofile);
		
		let mut max_threads: usize = 0;
		let mut split_by_subs: usize = 0;
		let mut	round_one_wait_interval: i64 = 0;
		let mut	input_path: String = String::new();
		let mut print_blocks: bool = false;
		let mut conserve_memory_at_cost_of_speed: bool = false;
		
		for line in file.lines().filter_map(|result| result.ok()) {				
	 		let mut config_item;
			let mut config_value; 
			let config_item_split = line.find(':');
			
 			match config_item_split {
				Some(x) =>{	
					config_item = line.slice(0,x);	
					config_value = line.slice(x+1,line.len());
					
					if config_item == "max_threads"{
						max_threads = config_value.as_slice().trim().parse::<usize>().unwrap();	
					} else if config_item == "split_by_subs"{
						split_by_subs = config_value.as_slice().trim().parse::<usize>().unwrap();	
					}else if config_item == "round_one_wait_interval_ns"{
						round_one_wait_interval = config_value.as_slice().trim().parse::<i64>().unwrap();					
					}else if config_item == "input_path"{
						input_path = config_value.as_slice().trim().to_string();
					} else if config_item == "print_blocks"{
						print_blocks = config_value.as_slice().trim().parse::<bool>().unwrap();
					} else if config_item == "conserve_memory_at_cost_of_speed"{
						conserve_memory_at_cost_of_speed = config_value.as_slice().trim().parse::<bool>().unwrap();
					} 
				}
				None => (),			
			}	
		}
		
		Config{max_threads:max_threads,round_one_wait_interval:round_one_wait_interval,input_path:input_path,print_blocks:print_blocks,conserve_memory_at_cost_of_speed:conserve_memory_at_cost_of_speed,split_by_subs:split_by_subs}
    }
}


struct Summary{
	block_name: String,
	block_size: usize,
	block_proteins: String,
	block_set_values: String,	
}
impl Summary{
	fn new()->Summary{
		Summary{block_name:String::new(),block_size:0,block_proteins:String::new(),block_set_values:String::new()}
	}

}
#[derive (Show,Clone)]
struct InputInfo{
	input_key: usize,
	input_values: HashSet<usize>,
    index: usize,
}
impl InputInfo{
	fn new()->InputInfo{
		InputInfo{input_key:0, input_values:HashSet::new(), index:0}
	}
}

#[derive(Show)]
struct SetInfo
{
	input_key_set: HashSet<uint>,
	intersect_set: HashSet<uint>,
}
impl SetInfo{
	fn new()->SetInfo{
		SetInfo{input_key_set:HashSet::new(), intersect_set:HashSet::new()}
	}
}

/* struct MTimer<'a> {
    name: &'a str,
    start: f64,	
} 

 impl<'a> MTimer<'a> {	
    fn new<'a>(name: &'a str) -> MTimer{
		MTimer{name: name, start: time::precise_time_s()}
    }
	
	fn stop(&self) -> f64{
		let diff  = time::precise_time_s() - self.start ;	
		println!("{} {}", self.name , diff );		
		diff	
	}
} */
	
fn main(){
	//let mut total_timer: MTimer = MTimer::new("Total Time");
	let config: Config = Config::new();		
	
	let mut timer = Timer::new().unwrap();	
	let set_counter = Arc::new(AtomicUsize::new(0));
	let mut main_map: Vec<InputInfo> = Vec::new();
	
	// Load input file
	load_input(&mut main_map,&config);	
	
	let shared_main_map = Arc::new(main_map);	// Use an Arc for memory efficiency when cloning	
  	let num_input_lines = shared_main_map.len();	
	 
	// Create hashmap and initialize with each protein as the key
	// p_map contains protein-set reference number
	let mut p_map_o: HashMap<usize,HashSet<usize>> = HashMap::new();
	for n in shared_main_map.iter(){	
		p_map_o.insert(n.input_key.clone(),HashSet::new());
	}	
	let p_map =  Arc::new(RwLock::new(p_map_o));

	let all_sets_o: HashMap<usize,SetInfo> = HashMap::new();
	let all_sets =  Arc::new(RwLock::new(all_sets_o));	
		
	let thread_info =  Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));	
	
	let mut max_threads = config.max_threads.clone();
	let mut start_at = 0;
	let mut count = 0;	
	let mut take;
     if num_input_lines <= max_threads { 
	  max_threads = 1;				  
	  take = num_input_lines //Take all items at once in this case
	} else {					
		take = num_input_lines / max_threads	// Take x number at a time						
	}	
	
	// Periodic timer to check active threads
	let mut periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval.clone()));
	
	for n in 0..num_input_lines{	
		loop {
			 periodic.recv().unwrap();
			{
				if thread_info.read().unwrap().active_count <= max_threads{
					//println!("Allow another thread to run - Active count {} Finished Count {} \n", thread_info.read().unwrap().active_count, thread_info.read().unwrap().finished_count );
					break;
				}
			} 
		} 
	
		let ref n_slice = shared_main_map[n];		
		do_intersection2(config.clone(), thread_info.clone(), all_sets.clone(), n_slice.index+1, shared_main_map.clone(), n_slice.clone());
						
	}
	// Extra check to wait for the last few threads to finish		
	periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval.clone()));
	loop {
		periodic.recv();	
		if thread_info.read().unwrap().finished_count == num_input_lines{
			break;
		}
	} 
	
	println!("Round One Done" );	
	
  	{
		let round_one_sets = all_sets.read().unwrap();
		for n in round_one_sets.iter(){
			println!("{:?}", n );
		} 
		println!("\n");
	}  
	
	//return;	
	let thread_info = Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));
	periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval.clone()));
	let num_sets = all_sets.read().unwrap().len();
	
	let round_one_sets = all_sets.read().unwrap();
	for (k,v) in round_one_sets.iter(){
		loop {
			 periodic.recv().unwrap();
			{
				if thread_info.read().unwrap().active_count <= max_threads{
					//println!("Allow another thread to run - Active count {} Finished Count {} \n", thread_info.read().unwrap().active_count, thread_info.read().unwrap().finished_count );
					break;
				}
			} 
		} 
	
		do_subs2(config.clone(), thread_info.clone(), all_sets.clone(), *k, v);
	}						

	// Extra check to wait for the last few threads to finish		
	periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval.clone()));
	loop {
		periodic.recv();	
		if thread_info.read().unwrap().finished_count == num_sets{
			break;
		}
	} 
			
/* 	{
		let roune_one_map = p_map.read().unwrap();
		for n in roune_one_map.iter(){
			println!("{:?}", n );
		}
		println!("\n");
	} */		

	
 	// Associate sub sets
	//t = MTimer::new("Round 2");	
/* 	let all_sets_o = all_sets.read().unwrap();
	let shared_all_sets = Arc::new(all_sets_o.clone());
	let thread_info = Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));	
	let mut split_by_subs = config.split_by_subs.clone();	
	start_at = 0;	
	take = if all_sets_o.len() <= split_by_subs { 
				  split_by_subs = 1;				  
				  all_sets_o.len() //Take all items at once in this case
				} else {					
					all_sets_o.len() / split_by_subs	// Take x number at a time						
				};
								
	while split_by_subs > 0{	
		//println!("subs start at {} take {}",start_at,take  );
		do_subs(config.clone(),thread_info.clone(),shared_all_sets.clone(),p_map.clone(), start_at, take);
		start_at = start_at + take;			
		split_by_subs-=1;
	}
	
	// Wait for all spawned threads to be finished
	// Check every 5 seconds	
	let periodic = timer.periodic(Duration::seconds(20));
	loop {
		periodic.recv();	
		//println!("{}-{}",thread_info.read().unwrap().finished_count, config.split_by_subs.clone() );
		if thread_info.read().unwrap().finished_count == config.split_by_subs.clone(){			
			break;
		}
	}  */
	//t.stop(); 	
	
	
/* 
 	t = MTimer::new("Round 2");
	// Go through each set to find sub sets
	// If a subset was found, add it to every protein's reference that had the superset ref
	{
		let all_sets_o = all_sets.read().unwrap();
		for (a_k,a_v) in all_sets_o.iter(){
			let a_len = a_k.len();	
			
			for (b_k,b_v) in all_sets_o.iter(){
				if a_len < b_k.len(){
					let sub_test: HashSet<usize> = a_k.clone().into_iter().collect();
					let super_test: HashSet<usize> = b_k.clone().into_iter().collect();
					
					if sub_test.is_subset(&super_test){
						//println!("Superset {:?} subset {:?}", super_test,sub_test  );					
						
						{						
							let mut p_map_o = p_map.write().unwrap();
							// For each HashSet<found sets>, see if the set contains the sub set reference
							for (p_value,p_sets) in p_map_o.iter_mut(){
								if p_sets.contains(b_v){
									// Insert the sub set reference						
									p_sets.insert(*a_v);
								
									//println!("Subset insert for protein {:?} subset {:?} of {:?} ", p_value,a_v,b_v );
								}
							}
						}					
					} 				
				}			
			}	
		}	
	}
	t.stop();	
	//println!("Round Two Done" );	 */
	
	//t = MTimer::new("Sort & Summarize");
	// Now find how many of each set there are along with the proteins that make up the set
	let mut lt_summary: Vec<Summary> = Vec::new();
	
	/* {
		let sets = all_sets.read().unwrap();
		let p_map_o = p_map.read().unwrap();
		
		for (s_k,s_v) in sets.iter(){			
			let mut proteins: HashSet<usize> = HashSet::new();
			
			for (p_k,p_v) in p_map_o.iter(){
				if p_v.contains(s_v){
					proteins.insert(*p_k);
				}				
			}
			
			let mut ls_summary: Summary = Summary::new();
			
			let mut block_name: String = proteins.len().to_string();		
			block_name.push_str("x");
			block_name.push_str(s_k.len().to_string().as_slice());
			
			ls_summary.block_name = block_name;
			ls_summary.block_size = proteins.len() * s_k.len();
			
			let protein_strings: Vec<String> = proteins.iter().map(|x| x.to_string()).collect();
			let set_strings: Vec<String> = s_k.iter().map(|x| x.to_string()).collect();
			
			ls_summary.block_proteins = protein_strings.connect(" ");
			ls_summary.block_set_values = set_strings.connect(" ");
			lt_summary.push(ls_summary);
			
			//println!("Set {:?} contains proteins {:?} and forms a {}x{} block", s_k, proteins,proteins.len() , s_k.len());		
		} 		
	}	
	
	// Print output or not
	if config.print_blocks{
		lt_summary.sort_by(|a,b| a.block_size.cmp(&b.block_size) );
		let mut prev_block_size: usize = 0;
		for n in lt_summary.iter(){
		
			if n.block_size != prev_block_size{
				println!("\n Block Size {}" , n.block_size);
			}
			
			println!("Name {} Proteins [{}] Set [{}] ", n.block_name, n.block_proteins,n.block_set_values );
			prev_block_size = n.block_size;
		}
	} */
	//t.stop();	

	//println!("Summary: Blocks Found {} \n", lt_summary.len() );
		
	//total_timer.stop();
	println!("Configuration: {:?}", config  );
	//infinite();		
}
 fn do_intersection2(config: Config, thread_info: Arc<RwLock<MyThreads>>, all_sets: Arc<RwLock<HashMap<usize,SetInfo>>>, skip: usize, lt_main_map: Arc<Vec<InputInfo>>, ls_main_map: InputInfo){	
	Thread::spawn( move || {			
   		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count +=1;
			//println!("Begin Thread - Started at {} take {} \n", start_at , take,);			
		}   
		 
		let mut l_all_sets: HashMap<usize,SetInfo> = HashMap::new();	
		
		//println!("\n loop a {}", a_k);		
		 for j in lt_main_map.iter().skip(skip){
			let mut intersection_vec: Vec<usize> = ls_main_map.input_values.intersection(&j.input_values).map(|&x| x).collect();				
			
			// Only keep the intersection if it made a "block" which is > 1 in length
			if intersection_vec.len() > 1{	
				//let mut intersection_vec: Vec<usize> = intersection.into_iter().collect();
				intersection_vec.sort(); // Sort the result in order
				let intersection_vec_hash = hash(&intersection_vec) as usize;
													
				//Conserve the most possible memory at the cost of speed
				if config.conserve_memory_at_cost_of_speed{
					// Store each unique set										
					{				
						let mut set = all_sets.write().unwrap();
						let mut new_set = false;							
						match set.get_mut(&intersection_vec_hash) {
							Some(x) => {
								x.input_key_set.insert(ls_main_map.input_key);		
								x.input_key_set.insert(j.input_key);								
							},
							None => {
								new_set = true;
							},
						}
						
						if new_set{
							let mut set_info = SetInfo::new();
							set_info.intersect_set = intersection_vec.clone().into_iter().collect();
							set_info.input_key_set.insert(ls_main_map.input_key);
							set_info.input_key_set.insert(j.input_key);
							set.insert(intersection_vec_hash,set_info);									
						}													
					}						
				} else{
					// Store each unique set	
					let mut new_set = false;						
					match l_all_sets.get_mut(&intersection_vec_hash) {
						Some(x) => {
							x.input_key_set.insert(ls_main_map.input_key);		
							x.input_key_set.insert(j.input_key);										
						},
						None => {
							new_set = true;
						},
					}
					
					if new_set{
						let mut set_info = SetInfo::new();
						set_info.intersect_set = intersection_vec.clone().into_iter().collect();
						set_info.input_key_set.insert(ls_main_map.input_key);
						set_info.input_key_set.insert(j.input_key);
						l_all_sets.insert(intersection_vec_hash,set_info);	
					}
				} 									
			}  
		}	
			
		//Put local data into global data
		if !config.conserve_memory_at_cost_of_speed{
			let mut set = all_sets.write().unwrap();
			
			for (intersection_vec_hash,v) in l_all_sets.drain(){
				let mut new_set = false;
				
				match set.get_mut(&intersection_vec_hash) {
					Some(x) => {					
						x.input_key_set = x.input_key_set.union(&v.input_key_set).map(|&x| x).collect();	
						x.intersect_set = x.intersect_set.union(&v.intersect_set).map(|&x| x).collect();
					},
					None => {
						new_set = true;
					},
				}
				
				if new_set{
					let mut set_info = SetInfo::new();
					set_info.intersect_set = v.intersect_set.clone();
					set_info.input_key_set = v.input_key_set.clone();						
					set.insert(intersection_vec_hash,set_info);	
				}				
			}
		}		
			

 		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count -=1;
			thread.finished_count +=1;
			//println!("End Thread - Started at {} take {} active threads {} \n", start_at , take, thread.active_count);
		} 	
		
	});		
}

 fn do_intersection(config: Config, thread_info: Arc<RwLock<MyThreads>>,set_counter: Arc<AtomicUsize>, all_sets: Arc<RwLock<HashMap<usize,SetInfo>>>, p_map: Arc<RwLock<HashMap<usize,HashSet<usize>>>>, start_at: usize, take: usize, main_map: Arc<Vec<InputInfo>>){	
	Thread::spawn( move || {			
   		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count +=1;
			//println!("Begin Thread - Started at {} take {} \n", start_at , take,);			
		}   
		 
		let mut b_count = start_at + 1;
		
		let mut l_all_sets: HashMap<usize,SetInfo> = HashMap::new();
		let mut l_p_map: HashMap<usize,HashSet<usize>> = HashMap::new();
 
		for i in main_map.iter().skip(start_at).take(take){	
		
			//println!("\n loop a {}", a_k);		
 			 for j in main_map.iter().skip(b_count){
				//println!("loop b {}",b_k);		
				//println!("loop b {} - loop a key {} - b_count {}  - start_at {}", b_k,a_k,b_count,start_at);	
				//println!("{} - {}",a_k,b_k);
				
		 	 	let mut intersection_vec: Vec<usize> = i.input_values.intersection(&j.input_values).map(|&x| x).collect();				
				
				// Only keep the intersection if it made a "block" which is > 1 in length
				if intersection_vec.len() > 1{	
					//let mut intersection_vec: Vec<usize> = intersection.into_iter().collect();
					intersection_vec.sort(); // Sort the result in order
					let intersection_vec_hash = hash(&intersection_vec) as usize;
														
  					//Conserve the most possible memory at the cost of speed
				    if config.conserve_memory_at_cost_of_speed{
						// Store each unique set										
						{				
							let mut set = all_sets.write().unwrap();
							let mut new_set = false;							
							match set.get_mut(&intersection_vec_hash) {
								Some(x) => {
									x.input_key_set.insert(i.input_key);		
								    x.input_key_set.insert(j.input_key);								
								},
								None => {
									new_set = true;
								},
							}
							
							if new_set{
								let mut set_info = SetInfo::new();
								set_info.intersect_set = intersection_vec.clone().into_iter().collect();
								set_info.input_key_set.insert(i.input_key);
								set_info.input_key_set.insert(j.input_key);
								set.insert(intersection_vec_hash,set_info);									
							}
														
						}						
					} else{
						// Store each unique set	
						let mut new_set = false;						
						match l_all_sets.get_mut(&intersection_vec_hash) {
							Some(x) => {
								x.input_key_set.insert(i.input_key);		
								x.input_key_set.insert(j.input_key);										
							},
							None => {
								new_set = true;
							},
						}
						
						if new_set{
							let mut set_info = SetInfo::new();
							set_info.intersect_set = intersection_vec.clone().into_iter().collect();
							set_info.input_key_set.insert(i.input_key);
							set_info.input_key_set.insert(j.input_key);
							l_all_sets.insert(intersection_vec_hash,set_info);	
						}
					} 									
				}  
			}			
			b_count+=1;				
		}
		
		//Put local data into global data
		if !config.conserve_memory_at_cost_of_speed{
			let mut set = all_sets.write().unwrap();
			
			for (intersection_vec_hash,v) in l_all_sets.drain(){
				let mut new_set = false;
				
				match set.get_mut(&intersection_vec_hash) {
					Some(x) => {					
						x.input_key_set = x.input_key_set.union(&v.input_key_set).map(|&x| x).collect();	
						x.intersect_set = x.intersect_set.union(&v.intersect_set).map(|&x| x).collect();
					},
					None => {
						new_set = true;
					},
				}
				
				if new_set{
					let mut set_info = SetInfo::new();
					set_info.intersect_set = v.intersect_set.clone();
					set_info.input_key_set = v.input_key_set.clone();						
					set.insert(intersection_vec_hash,set_info);	
				}				
			}
		}		
			

 		{
			let mut thread = thread_info.write().unwrap();
			//thread.active_count -=1;
			thread.finished_count +=1;
			//println!("End Thread - Started at {} take {} active threads {} \n", start_at , take, thread.active_count);
		} 	
		
	});		
}

 fn do_subs2(config: Config, thread_info: Arc<RwLock<MyThreads>>, all_sets: Arc<RwLock<HashMap<usize,SetInfo>>>, a_set_key: usize, a_set_info: SetInfo){	
		
	Thread::spawn( move || {		
		
		//Get length of the set
		let a_len = a_set_info.intersect_set.len();	
		
		//Tmp hashmap of <Hash of Set, Set of Keys>
		let mut tmp: HashMap<usize,HashSet<usize>> = HashMap::new();
					
		{
			let lt_all_sets = all_sets.read().unwrap();	

			for (b_k,b_v) in lt_all_sets.iter(){
				if a_len < b_v.intersect_set.len(){			
					// Check if one set is a subset of another
					if a_set_info.intersect_set.is_subset(&b_v.intersect_set){					
						{	
							//Take the union of the key sets
							let mut new_set = false;						
							match tmp.get_mut(&a_set_key) {
								Some(x) => {
									let mut a_key_union: HashSet<usize> = a_set_info.input_key_set.union(&b_v.input_key_set).map(|&x| x).collect();	
									
									//x = x.union(&a_key_union).map(|&x| x).collect();								
								},
								None => {
									new_set = true;
								},
							}
							
							if new_set{
								tmp.insert(a_set_key,a_set_info.input_key_set.union(&b_v.input_key_set).map(|&x| x).collect());							
							}						
						}					
					} 				
				}			
			}
		}		
		
		{			
			let mut set = all_sets.write().unwrap();
			
			for (k,v) in tmp.iter(){
				let mut new_set = false;
				
				match set.get_mut(k) {
					Some(x) => {
						//let union = x.input_key_set.union(v).map(|&x| x).collect();	
						//x.input_key_set = x.input_key_set.union(v).map(|&x| x).collect();								
					},
					None => {
						panic!("This should never happen - Did not find key of set");						
					},
				}				
			}			
		}			

 		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count -=1;
			thread.finished_count +=1;
			//println!("End Thread - Started at {} take {} active threads {} \n", start_at , take, thread.active_count);
		} 	
		
	});		
}


 fn do_subs(config: Config, thread_info: Arc<RwLock<MyThreads>>, all_sets: Arc<HashMap<Vec<usize>,usize>>, p_map: Arc<RwLock<HashMap<usize,HashSet<usize>>>>, start_at: usize, take: usize){	
	Thread::spawn( move || {	
		let mut l_p_map = p_map.read().unwrap().clone();	
	
		//let all_sets_o = all_sets.read().unwrap();
		for (a_k,a_v) in all_sets.iter().skip(start_at).take(take){
			let a_len = a_k.len();	
			
			for (b_k,b_v) in all_sets.iter(){
				if a_len < b_k.len(){
					let sub_test: HashSet<usize> = a_k.clone().into_iter().collect();
					let super_test: HashSet<usize> = b_k.clone().into_iter().collect();
					
					if sub_test.is_subset(&super_test){
						//println!("Superset {:?} subset {:?}", super_test,sub_test  );					
						
						{						
							//let mut p_map_o = p_map.write().unwrap();
							// For each HashSet<found sets>, see if the set contains the sub set reference
							for (p_value,p_sets) in l_p_map.iter_mut(){
								if p_sets.contains(b_v){
									// Insert the sub set reference						
									p_sets.insert(*a_v);
								
									//println!("Subset insert for protein {:?} subset {:?} of {:?} ", p_value,a_v,b_v );
								}
							}
						}					
					} 				
				}			
			}	
		}
		
		{
			let mut p_map_o = p_map.write().unwrap();
			for (p_value,p_sets) in l_p_map.drain(){
				match p_map_o.get_mut(&p_value) {
					Some(x) => *x = p_sets,
					None => (),
				}			
			}	
		}		

 		{
			let mut thread = thread_info.write().unwrap();
			//thread.active_count -=1;
			thread.finished_count +=1;
			//println!("End Thread - Started at {} take {} active threads {} \n", start_at , take, thread.active_count);
		} 	
		
	});		
}


fn load_input(main_map: &mut Vec<InputInfo>, config:&Config){
	// Create a path to the desired file
    let path = Path::new(config.input_path.clone());
	let display = path.display();	
	
	let ofile = match File::open_mode(&path, Open, ReadWrite) {
		Ok(f) => f,
		Err(e) => panic!("input file error: {} {}", display,e.desc),
	};
		
	//let t = MTimer::new("Load Input File");
	let mut index = 0;
	let mut file = BufferedReader::new(ofile);
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
		
		let mut input_info = InputInfo::new();
		input_info.input_key = protein.parse::<usize>().unwrap();
		input_info.input_values = c_set;
		input_info.index = index;
		main_map.push(input_info);
		
		index+=1;	
		
		//main_map.insert(protein.parse::<usize>().unwrap(),c_set);	
	}
	//t.stop();
}


fn infinite(){
	loop {	
	}
}

//extern crate time;
extern crate xxhash;
extern crate natord;
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
		
		Config{max_threads:max_threads,round_one_wait_interval:round_one_wait_interval,input_path:input_path,print_blocks:print_blocks,conserve_memory_at_cost_of_speed:conserve_memory_at_cost_of_speed}
    }
}


struct Summary{
	block_name: String,
	block_size: usize,
	block_keys: String,
	block_values: String,	
}
impl Summary{
	fn new()->Summary{
		Summary{block_name:String::new(),block_size:0,block_keys:String::new(),block_values:String::new()}
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

#[derive(Show,Clone)]
struct SetInfo
{
	input_key_set: HashSet<usize>,
	intersect_set: HashSet<usize>,
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
	
	// Load input file
	let mut main_map: Vec<InputInfo> = Vec::new();	
	load_input(&mut main_map,&config);	
	
	let shared_main_map = Arc::new(main_map);	// Use an Arc for memory efficiency when cloning	
  	let num_input_lines = shared_main_map.len();	

	let all_sets_o: HashMap<usize,SetInfo> = HashMap::new();
	let all_sets =  Arc::new(RwLock::new(all_sets_o));	
		
	let thread_info =  Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));	

	// Periodic timer to check active threads
	let mut periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval));
		
	for n in 0..num_input_lines{	
		loop {
			 periodic.recv().unwrap();
			{
				if thread_info.read().unwrap().active_count <= config.max_threads{
					//println!("Allow another thread to run - Active count {} Finished Count {} \n", thread_info.read().unwrap().active_count, thread_info.read().unwrap().finished_count );
					break;
				}
			} 
		} 
		
		println!("Doing number {}", n );
	
		let ref n_slice = shared_main_map[n];		
		do_intersection(config.clone(), thread_info.clone(), all_sets.clone(), n_slice.index+1, shared_main_map.clone(), n_slice.clone());
						
	}
	// Extra check to wait for the last few threads to finish		
	periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval));
	loop {
		periodic.recv();	
		if thread_info.read().unwrap().finished_count == num_input_lines{
			break;
		}
	} 
	
	println!("Round One Done" );	
	
/*   	{
		let round_one_sets = all_sets.read().unwrap();
		for n in round_one_sets.iter(){
			println!("{:?}", n );
		} 
		println!("\n");
	}   */
	
	//return;	
	let filled_sub_sets_o: HashMap<usize,HashSet<usize>> = HashMap::new();
	let filled_sub_sets =  Arc::new(RwLock::new(filled_sub_sets_o));
	
	let thread_info = Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));
	let num_sets = all_sets.read().unwrap().len();
	
	let mut periodic = timer.periodic(Duration::nanoseconds(10));
	{
		let round_one_sets = all_sets.read().unwrap();

		for (k,v) in round_one_sets.iter(){	
			loop {
				 periodic.recv().unwrap();
				{
					if thread_info.read().unwrap().active_count <= config.max_threads{
						//println!("Allow another thread to run - Active count {} Finished Count {} \n", thread_info.read().unwrap().active_count, thread_info.read().unwrap().finished_count );
						break;
					}
				} 
			} 
		
			do_subs(thread_info.clone(), all_sets.clone(), *k, v.clone(), filled_sub_sets.clone());
		}	
    }	

	
	// Extra check to wait for the last few threads to finish		
	periodic = timer.periodic(Duration::nanoseconds(config.round_one_wait_interval));
	loop {
		periodic.recv();	
		if thread_info.read().unwrap().finished_count == num_sets{
			break;
		}
	} 
	
	println!("Round Two Done" );
	
	// Update sets with the filled subset keys
	 {		 
		let filled_subsets = filled_sub_sets.read().unwrap();		
		let mut set = all_sets.write().unwrap();	
		
		for (sub_k,sub_v) in filled_subsets.iter(){
			match set.get_mut(sub_k) {
				Some(x) => {
					let upd_union: HashSet<usize> = x.input_key_set.union(sub_v).map(|&x| x).collect();
					x.input_key_set = upd_union;						
				},
				None => {
					panic!("This should never happen");					
				},
			}						
		} 		
	}	 
	
	println!("Merge Done" );	
	
	//t = MTimer::new("Sort & Summarize");
	// Now find how many of each set there are along with the proteins that make up the set
	let mut lt_summary: Vec<Summary> = Vec::new();
	
	{
		let sets = all_sets.read().unwrap();
			
		for (s_k,s_v) in sets.iter(){	
			let mut ls_summary: Summary = Summary::new();
			
			let mut block_name: String = s_v.input_key_set.len().to_string();		
			block_name.push_str("x");
			block_name.push_str(s_v.intersect_set.len().to_string().as_slice());
			
			ls_summary.block_name = block_name;
			ls_summary.block_size = s_v.input_key_set.len() * s_v.intersect_set.len();
			
			let mut key_strings: Vec<String> = s_v.input_key_set.iter().map(|x| x.to_string()).collect();
			let mut value_strings: Vec<String> = s_v.intersect_set.iter().map(|x| x.to_string()).collect();
			
			//Sort each set	
			key_strings.sort_by(|a,b| natord::compare(a,b));
			value_strings.sort_by(|a,b| natord::compare(a,b));			
			
			ls_summary.block_keys = key_strings.connect(" ");
			ls_summary.block_values = value_strings.connect(" ");
			lt_summary.push(ls_summary);
			
			//println!("Set {:?} contains proteins {:?} and forms a {}x{} block", s_k, proteins,proteins.len() , s_k.len());		
		} 		
	}	
	
	println!("Summary Done" );
	
	// Print output or not
 	if config.print_blocks{
		lt_summary.sort_by(|a,b| a.block_size.cmp(&b.block_size) );
		let mut prev_block_size: usize = 0;
		for n in lt_summary.iter(){
		
			if n.block_size != prev_block_size{
				//println!("\n Block Size {}" , n.block_size);
			}
			
			println!("C[{}] P[{}]", n.block_values,n.block_keys );
			prev_block_size = n.block_size;
		}
	}  
	//t.stop();	

	println!("Summary: Blocks Found {} \n", lt_summary.len() );
		
	//total_timer.stop();
	println!("Configuration: {:?}", config  );			
}
 fn do_intersection(config: Config, thread_info: Arc<RwLock<MyThreads>>, all_sets: Arc<RwLock<HashMap<usize,SetInfo>>>, skip: usize, lt_main_map: Arc<Vec<InputInfo>>, ls_main_map: InputInfo){	
	Thread::spawn( move || {			
   		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count +=1;			
		}   
		 
		let mut l_all_sets: HashMap<usize,SetInfo> = HashMap::new();	
		
		
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
		} 	
		
	});		
}

 fn do_subs(thread_info: Arc<RwLock<MyThreads>>, all_sets: Arc<RwLock<HashMap<usize,SetInfo>>>, a_set_key: usize, a_set_info: SetInfo, filled_sub_sets: Arc<RwLock<HashMap<usize,HashSet<usize>>>>){	
		
	Thread::spawn( move || {
   		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count +=1;					
		}  	
		
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
									let upd_union: HashSet<usize> = x.union(&b_v.input_key_set).map(|&x| x).collect();	
									*x = upd_union;								
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
			// Add filled sub set
			let mut set = filled_sub_sets.write().unwrap();
						
			for (k,v) in tmp.drain(){	
				let mut new_set = false;				
				match set.get_mut(&k) {
					Some(x) => {
						let upd_union: HashSet<usize> = x.union(&v).map(|&x| x).collect();
						*x = upd_union;						
					},
					None => {
						new_set = true;						
					},
				}

				if new_set{
					set.insert(k,v);								
				}	
			}			
		}	 	

 		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count -=1;
			thread.finished_count +=1;			
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
	}
	//t.stop();
}


fn infinite(){
	loop {	
	}
}

extern crate time;
use std::io::{File, Open, ReadWrite};
use std::io::BufferedReader;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::io::Timer;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

struct MyThreads{
	finished_count: usize,
	active_count: usize,
}

#[derive (Show,Clone)]
struct Config{
	split_by: usize,
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
		
		let mut split_by: usize = 0;
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
					
					if config_item == "split_by"{
						split_by = config_value.as_slice().trim().parse::<usize>().unwrap();	
					}else if config_item == "round_one_wait_interval_s"{
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
		
		Config{split_by:split_by,round_one_wait_interval:round_one_wait_interval,input_path:input_path,print_blocks:print_blocks,conserve_memory_at_cost_of_speed:conserve_memory_at_cost_of_speed}
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

struct MTimer<'a> {
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
}
	
fn main(){
	let mut total_timer: MTimer = MTimer::new("Total Time");
	let config: Config = Config::new();	
	
	let mut timer = Timer::new().unwrap();	
	let set_counter = Arc::new(AtomicUsize::new(0));
	let mut main_map = HashMap::new();	
	
	// Load input file
	load_input(&mut main_map,&config);	
	let shared_main_map = Arc::new(main_map);		
  	let num_items = shared_main_map.len();
	
	// Create hashmap and initialize with each protein as the key
	// p_map contains protein-set reference number
	let mut p_map_o: HashMap<usize,HashSet<usize>> = HashMap::new();
	for (k,v) in shared_main_map.iter(){		
		p_map_o.insert(k.clone(),HashSet::new());
	}	
	let p_map =  Arc::new(RwLock::new(p_map_o));		
	
	// Create hashmap of set-reference number
	let all_sets_o: HashMap<Vec<usize>,usize> = HashMap::new();
	let all_sets =  Arc::new(RwLock::new(all_sets_o));	
		
	let thread_info =  Arc::new(RwLock::new(MyThreads{active_count:0,finished_count:0}));	
	
	let mut split_by = config.split_by.clone();
	let mut start_at = 0;
	let mut count = 0;	
	let take = if num_items <= split_by { 
				  split_by = 1;				  
				  num_items //Take all items at once in this case
				} else {					
					num_items / split_by	// Take x number at a time						
				};
	
	// Number of threads that can be running at a time
	//let thread_limiter = config.thread_limit;	
			
	let mut t: MTimer = MTimer::new("Round 1");
	while split_by > 0{	
		// This loop is only executed once every x seconds and checks how many threads are running now
		// If the number is less than the threshold, allow another thread to run
/*  		loop {
			periodic.recv().unwrap();
			if thread_info.read().unwrap().active_count <= thread_limiter{
				//println!("Allow another thread to run - Active count {} Finished Count {} \n", thread_info.read().unwrap().active_count //,thread_info.read().unwrap().finished_count );
				break;
			}			
		} 	 */	
		

		// Do intersection		
		do_intersection(config.clone(),thread_info.clone(),set_counter.clone(),all_sets.clone(),p_map.clone(), start_at, take, shared_main_map.clone());
		
		start_at = start_at + take;			
		split_by-=1;
	}
	
	// Wait for all spawned threads to be finished
	// Check every 5 seconds	
	let periodic = timer.periodic(Duration::seconds(config.round_one_wait_interval.clone()));
	loop {
		periodic.recv();	
		if thread_info.read().unwrap().finished_count == config.split_by.clone(){
			break;
		}
	} 
	t.stop();
		
	//println!("Round One Done" );	
	
/*  	{
		let roune_one_sets = all_sets.read().unwrap();
		for n in roune_one_sets.iter(){
			println!("{:?}", n );
		} 
		println!("\n");
	}  */
/* 	{
		let roune_one_map = p_map.read().unwrap();
		for n in roune_one_map.iter(){
			println!("{:?}", n );
		}
		println!("\n");
	} */		


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
	//println!("Round Two Done" );	
	
	t = MTimer::new("Sort & Summarize");
	// Now find how many of each set there are along with the proteins that make up the set
	let mut lt_summary: Vec<Summary> = Vec::new();
	
	{
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
	}
	t.stop();	

	println!("Summary: Blocks Found {} \n", lt_summary.len() );
		
	total_timer.stop();
	println!("Configuration: {:?}", config  );
	//infinite();		
}

 fn do_intersection(config: Config, thread_info: Arc<RwLock<MyThreads>>,set_counter: Arc<AtomicUsize>, all_sets: Arc<RwLock<HashMap<Vec<usize>,usize>>>, p_map: Arc<RwLock<HashMap<usize,HashSet<usize>>>>, start_at: usize, take: usize, main_map: Arc<HashMap<usize,HashSet<usize>>>){	
	Thread::spawn( move || {	
/*  		{
			let mut thread = thread_info.write().unwrap();
			thread.active_count +=1;
			//println!("Begin Thread - Started at {} take {} \n", start_at , take,);			
		} 
		 */
		let mut b_count = start_at + 1;
		
		let mut l_all_sets: HashMap<Vec<usize>,usize> = HashMap::new();
		let mut l_p_map: HashMap<usize,HashSet<usize>> = HashMap::new();
 
		for (a_k, a_v) in main_map.iter().skip(start_at).take(take){		
			//println!("\n loop a {}", a_k);		
 			 for (b_k, b_v) in main_map.iter().skip(b_count){
				//println!("loop b {}",b_k);		
				//println!("loop b {} - loop a key {} - b_count {}  - start_at {}", b_k,a_k,b_count,start_at);	
				//println!("{} - {}",a_k,b_k);
				
		 	 	let intersection : HashSet<usize> = a_v.intersection(b_v).map(|&x| x).collect();				
				
				// Only keep the intersection if it made a "block" which is > 1 in length
				if intersection.len() > 1{	
					let mut intersection_vec: Vec<usize> = intersection.into_iter().collect();
					intersection_vec.sort(); // Sort the result in order
														
  					//Conserve the most possible memory at the cost of speed
				    if config.conserve_memory_at_cost_of_speed{
						// Store each unique set
						let mut next_num = 0;					
						{				
							let mut set = all_sets.write().unwrap();							
							if	!set.contains_key(&intersection_vec){
								next_num = set_counter.fetch_add(1,Ordering::Relaxed);											
								set.insert(intersection_vec.clone(),next_num);												
							}
							
							if next_num == 0{
								match set.get(&intersection_vec.clone()){
									Some(set_ref) => {
										next_num = *set_ref;								
									},
									None => {},					
								}
							} 
						}
						
						{
							//Associate each protein with the set that was found
							let mut p_set = p_map.write().unwrap();	
							match p_set.get_mut(a_k){
								Some(p_refs) => {
									p_refs.insert(next_num);									
								},
								None => {},					
							}	
							match p_set.get_mut(b_k){
								Some(p_refs) => {
									p_refs.insert(next_num);									
								},
								None => {},					
							}  
						}
					} else{
						// Store each unique set	
						let mut next_num = 0;					
						{									
							if	!l_all_sets.contains_key(&intersection_vec){
								next_num = l_all_sets.len() + 1;
								l_all_sets.insert(intersection_vec.clone(),next_num);												
							}
							
							if next_num == 0{
								match l_all_sets.get(&intersection_vec.clone()){
									Some(set_ref) => {
										next_num = *set_ref;								
									},
									None => {},					
								}
							} 
						}
						
						{
							//Associate each protein with the set that was found
							let mut insert: bool = false;
							match l_p_map.get_mut(a_k){
								Some(p_refs) => {
									p_refs.insert(next_num);									
								},
								None => {
									insert = true;													
								},					
							}	
							if insert{
								let mut set: HashSet<usize> = HashSet::new();
								set.insert(next_num);
								l_p_map.insert(*a_k,set);	
							}
							insert = false;
							
							match l_p_map.get_mut(b_k){
								Some(p_refs) => {
									p_refs.insert(next_num);									
								},
								None => {
									insert = true;		
								},					
							}
							
							if insert{
								let mut set: HashSet<usize> = HashSet::new();
								set.insert(next_num);
								l_p_map.insert(*b_k,set);	
							}	
						}
					} 									
				}  
			}			
			b_count+=1;				
		}
		
		//Put local data into global data
		if !config.conserve_memory_at_cost_of_speed{			
			let mut next_num = 0;					
			{		
				// Unique sets
				let mut set = all_sets.write().unwrap();		
				for (k,v) in l_all_sets.drain(){
					if	!set.contains_key(&k){
						next_num = set_counter.fetch_add(1,Ordering::Relaxed);											
						set.insert(k.clone(),next_num);												
					}else{								
						match set.get(&k){
							Some(set_ref) => {							
								next_num = *set_ref;								
							},
							None => {},					
						}					
					}
					
					// Protein-Set mapping
					for (pk,pv) in l_p_map.iter(){
						if pv.contains(&v){
							{
								//Associate each protein with the set that was found
								let mut p_set = p_map.write().unwrap();	
								match p_set.get_mut(pk){
									Some(p_refs) => {
										p_refs.insert(next_num);									
									},
									None => {},					
								}							 
							}
						}					
					}								
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


fn load_input(main_map: &mut HashMap<usize,HashSet<usize>>, config:&Config){
	// Create a path to the desired file
    let path = Path::new(config.input_path.clone());
	let display = path.display();	
	
	let ofile = match File::open_mode(&path, Open, ReadWrite) {
		Ok(f) => f,
		Err(e) => panic!("input file error: {} {}", display,e.desc),
	};
		
	let t = MTimer::new("Load Input File");	

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
		
		main_map.insert(protein.parse::<usize>().unwrap(),c_set);	
	}
	t.stop();
}


fn infinite(){
	loop {	
	}
}

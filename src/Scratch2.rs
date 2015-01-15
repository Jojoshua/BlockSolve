use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Arc;

fn main() {
    let mut main_map = HashMap::new();
    for n in range(0,10){
        main_map.insert(n, n+1);		
    }
	for (k,v) in main_map.iter(){
		println!("{}", k);  	
	}
    println!("\n");  
	
    let shared_main_map = Arc::new(main_map);
    let num_items = shared_main_map.len();
    
    for num in range(0,num_items){        
		do_work(num,shared_main_map.clone());   
	}  
	
	loop{
	}
    
}

fn do_work(start_at: usize, main_map: Arc<HashMap<usize,usize>>){	
	Thread::spawn( move || {
		for (a_k, _) in main_map.iter().skip(start_at).take(1){
			//println!("top loop {}", a_k);  
			
			for (b_k, b_v) in main_map.iter().skip(start_at + 1){
				println!("bottom loop {} {}", a_k, b_k);  
			}			
		} 
	});	
}	

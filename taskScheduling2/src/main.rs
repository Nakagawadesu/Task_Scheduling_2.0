mod Graphs;

use petgraph::graph;

use std::time::{Instant, Duration};
fn main() {



    let mut n_ants = 8;
    let mut file_path ="";
    let mut graph_name ="";

        file_path = "/home/matheus/STG/protostg/";
        graph_name = "atest.stg";
    
        //let file_path = "/home/matheus/STG/500/";
        //let graph_name = "rand0147.stg";
    

    let mut graph = Graphs::Utils::new();


   // ## INITIALIZATION FOR PROTOTYPE GRAPH
    graph.initialize_graph_prototype(file_path,graph_name, &mut n_ants);

   // ## INITIALIZATION FOR NORMAL GRAPH
   //graph.initialize_graph(file_path,graph_name);
    
    
    graph.print_graph();
    // Futeher Initializations could be in a function
    let n_tasks = graph.di_graph.node_count();
    graph.find_max_cost_unlocks(n_tasks);
    let visibility_init = graph.update_visibility(n_tasks);
    graph.print_remaining_vec(n_tasks);
    
    let start_time = Instant::now();
 
  

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);
    let elapsed_seconds = elapsed_time.as_secs();
    let elapsed_millis = elapsed_time.as_millis();
    let elapsed_micros = elapsed_time.as_micros();

   

}

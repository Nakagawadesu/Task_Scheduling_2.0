mod Graphs;

use std::time::{Instant};
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
    graph.print_visibility(n_tasks ,&visibility_init);
    let start_time = Instant::now();
 
  /*
    let mut colony = Ants::Army::Colony::new(n_tasks ,0.1);
    colony.update_visibility_sum();
    let mut worker = Ants::ManagerAnt::new(0.2, &graph.remaining_vec);
    let sequence = worker.work(&mut graph ,n_ants,&mut colony);
*/   

    let end_time = Instant::now();

    let elapsed_time = end_time.duration_since(start_time);
    let elapsed_seconds = elapsed_time.as_secs();
    let elapsed_millis = elapsed_time.as_millis();
    let elapsed_micros = elapsed_time.as_micros();

    println!(" Sequence:");
    for i in 0..n_tasks {
        //print!(" {}",sequence[i]);
        print!(" {}",aco.optimal_schedule[i]);
    }
println!(" ");    
println!(
    "{} Ants spent : {} , computer spent {} micro seconds",
n_ants,
//worker.time_spent ,
aco.optimal_time, 
elapsed_micros
);
graph.write_results_to_file(
    "/home/matheus/STG/Results/",
    &graph_name ,
    &aco.optimal_schedule , 
    &aco.optimal_time,
    &n_ants,
    &n_colonies,
    &pherohormones_intensity,
    &benchmark
);


}

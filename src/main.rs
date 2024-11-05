mod Ants;
mod Graphs;
use std::time::Instant;
fn main() {
    let mut n_ants: i128 = 8;
    let mut file_path: &str = "";
    let mut graph_name: &str = "";

    file_path = "/home/matheus/STG/protostg/";
    graph_name = "atest.stg";

    //let file_path = "/home/matheus/STG/500/";
    //let graph_name = "rand0147.stg";

    let mut graph: Graphs::Utils = Graphs::Utils::new();

    // ## INITIALIZATION FOR PROTOTYPE GRAPH
    graph.initialize_graph_prototype(file_path, graph_name, &mut n_ants);

    // ## INITIALIZATION FOR NORMAL GRAPH
    //graph.initialize_graph(file_path,graph_name);

    graph.print_graph();
    // Futeher Initializations could be in a function
    let n_tasks: usize = graph.di_graph.node_count();
    graph.find_max_cost_unlocks(n_tasks);
    let visibility_init: Vec<f64> = graph.update_visibility(n_tasks);
    graph.print_remaining_vec(n_tasks);
    graph.print_visibility(n_tasks, &visibility_init);
    let start_time: Instant = Instant::now();

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
}

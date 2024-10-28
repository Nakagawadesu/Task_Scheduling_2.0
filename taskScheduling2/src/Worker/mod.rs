
pub mod Tuple;
pub mod Worker;
pub mod Army;

use petgraph::stable_graph::{StableDiGraph, NodeIndex};
use std::collections::BinaryHeap; 
use petgraph::Direction;
use rand::Rng;

use crate::Graphs::Utils;
use Tuple::TaskTuple;
use Worker::WorkerAnt;
use Army::Colony;

pub(crate) struct ManagerAnt {
    pub(crate) time_spent: i128,
    pub(crate) task_heap: BinaryHeap<TaskTuple>,
    pub(crate) remaining_vec: Vec<i128>,
    pub(crate) randomness: f64
    
}

impl ManagerAnt {
    pub fn new( vec : &Vec<i128> , rand : f64) -> Self {
        Self {
            time_spent: 0,
            remaining_vec: vec.clone(),
            randomness : rand

        }
    }
    pub fn scan_available_tasks(
        &mut self ,
        colony : &Colony,
        generation: i128,
        colonies : i128
        ){
        for i in 0..self.remaining_vec.len(){
            if self.remaining_vec[i as usize] == 0 {
                let mut inserted : TaskTuple = TaskTuple::new(NodeIndex::new(i as usize) , 0.0);
                
                //randomness
                let mut rng = rand::thread_rng();
                let random_float: f64 = rng.gen_range(0.0..self.randomness);
                //calculate params
                //Priority(s,t) = ( phe(t) ^a * vis(t) ^b ) / ( Sum(phe(t))^a * Sum( vis(t)^b ) 
                // + Rand
                let index = inserted.node.index();
                let a = colony.a;
                let w = colony.w;
                let mut aco_metrics = ( colony.pherohormones[index].powf(a) * colony.visibility[index].powf(w) ) / (colony.pherohormones_sum.powf(a)* colony.visibility_sum.powf(w) );
                let mut random_metrics = 0.0;
                
                
             
              //# INSERT IN A QUEUE OR SOMETHING ALIKE
              //  self.task_heap.push(inserted);
            }
        }
        
    }
    pub fn reduce_neighbors(
        &mut self ,
        clone : &mut StableDiGraph<i128,i128>,
        task: &TaskTuple,
        colony : &Colony,
        generation: i128,
        colonies : i128
        ){

        let mut neighbors : Vec<i128> = Vec::new() ;
        for neighbor in clone.neighbors_directed( task.node, Direction::Outgoing) {
            let neighbor_index = clone[neighbor];
            neighbors.push(neighbor_index);
        }
        //println!("task : {} , neghbors :\n",index);

        
        let remaining_vec = &mut self.remaining_vec;
        for i in neighbors{
            remaining_vec[i as usize] = remaining_vec[i as usize] -1;
            //println!("reduced {}",i);
            //self.print_remaining_vec(&remaining_vec);
            if remaining_vec[i as usize] == 0 {
                
                //randomness
                let mut rng = rand::thread_rng();
                let random_float: f64 = rng.gen_range(0.0..self.randomness);
                //calculate params
                //Priority(s,t) = ( phe(t) ^a * vis(t) ^b ) / ( Sum(phe(t))^a * Sum( vis(t)^b ) 
                // + Rand
                let index = inserted.node.index();
                let a = colony.a;
                let w = colony.w;
                let mut aco_metrics = ( colony.pherohormones[index].powf(a) * colony.visibility[index].powf(w) ) / (colony.pherohormones_sum.powf(a)* colony.visibility_sum.powf(w) );
                let mut random_metrics = 0.0;
           // make it open to be done
                
            }
        }
        
        
    }

    fn choose_task(&mut self ) -> Option<TaskTuple>{
    
    }
    

    pub fn remove_edges(&self ,graph: &mut StableDiGraph<i128,i128>, task: NodeIndex) {
        let neighbors: Vec<NodeIndex> = graph.neighbors(task).collect();

        for neighbor in neighbors {
            if let Some(edge) = graph.find_edge(task, neighbor) {
                graph.remove_edge(edge);
            }
        }
    }

    pub fn complete_task(
        &mut self,
        clone : &mut StableDiGraph<i128,i128>,
        task: &TaskTuple, 
        sequence : &mut Vec<i128>,
        workers: &mut Vec<WorkerAnt>   ,
        worker : usize,
        colony : &mut Colony,
        iteration : &mut i128,
        generation : &mut i128,
        colonies : &mut i128
    ){

        self.reduce_neighbors(clone,task,&colony,*generation , *colonies);
        self.remove_edges(clone, task.node);
        clone.remove_node(task.node);
        sequence.push(task.node.index().try_into().unwrap());
        colony.add_pherohormones(  *iteration , task.node.index() );
        *iteration += 1;
       // println!("task {} completed", task.node.index());
    }
    

    pub fn work(
        &mut self , 
        graph : &mut Utils,
        n_workers: i128,
        colony : &mut Colony,
        gen : i128,
        n_colonies: i128,
        is_proto: i128
        )->Vec<i128>{

        let mut sequence :Vec<i128> = Vec::new();
        let mut task_tuple : TaskTuple;  
        let mut workers: Vec<WorkerAnt> = vec![WorkerAnt::new(-1); n_workers as usize];
        let mut iteration : i128 = 0;
        let mut generation = gen;
        let mut colonies = n_colonies;
        let mut clone = graph.di_graph.clone();
        
        if is_proto == 0 {
            task_tuple = TaskTuple::new(NodeIndex::new(0),0.0);
            self.complete_task(
                &mut clone, 
                &task_tuple, 
                &mut sequence,
                &mut workers, 
                0 as usize,
                colony,
                &mut iteration, 
                &mut generation ,
                &mut colonies);
        } else {
            self.scan_available_tasks(colony , generation , colonies);   
        }

        


        while clone.node_count() > 0 {
           for i in 0..n_workers{
                //println!("n_worker : {} , current worker : {} ",n_workers, i);
                let index = i as usize;

                //println!("worker: {}, spent: {} , ",i as i128,self.time_spent);
                if workers[index].free_at <= self.time_spent {
                    
                    //liberação do worker
                    if workers[index].free_at <= self.time_spent && workers[index].current_task != -1{
                        task_tuple = TaskTuple::new(NodeIndex::new(workers[index].current_task as usize),0.0);
                        //println!("finished: {}",workers[index].current_task );
                        self.complete_task(
                            &mut clone, 
                            &task_tuple, 
                            &mut sequence, 
                            &mut workers, 
                            index, 
                            colony,
                            &mut iteration,
                            &mut generation, 
                            &mut colonies);
                        workers[index].current_task = -1;
                       
                    }
                    //se liberado botar para trabalhar
                    if  workers[index].current_task == -1 {
                        if let Some(task) = self.choose_task() {
                            workers[index].start_task(
                                &self.time_spent , 
                                &(task.node.index() as i128),
                                &graph.costs_vec[task.node.index() as usize],
                                &is_proto
                            );
                            /*
                            println!("worker :  {} , started at : {} , finishes at : {}, task: {}",
                            i,
                            self.time_spent,
                            workers[index].free_at,
                            workers[index].current_task,
                   );*/
                    } else {
                           // println!("worker {} waiting,  current time spent {}", i, self.time_spent);
                        }
                    }
                    
                }
           } 
           //print!("current time : {}",self.time_spent);
           self.time_spent += 1;
        }
        //grafo vazio poré podem ter tarefas ainda sendo executadas
        let mut last_finished : i128 = -1;
        for i in 0..n_workers{
            if last_finished < workers[i as usize].free_at{
                last_finished = workers[i as usize].free_at;
                
            }
        }
        self.time_spent = last_finished;    
        sequence
    }

    
    pub fn print_remaining_vec(&self, remaining_vec: &Vec<i128>) {
        println!(" Remaining:");
        for (index, value) in remaining_vec.iter().enumerate() {
            println!(" {}: {}", index, value);
        }
    }
    
}
use petgraph::stable_graph::{StableDiGraph, NodeIndex, EdgeIndex};

pub(crate) struct Colony{
    pub(crate) pherohormones: StableDiGraph <i128, i128>,
    pub(crate) visibility :Vec<f64>,
    pub(crate) pherohormones_intensity: f64,
    pub(crate) evaporation : i128,
    pub(crate) n_tasks: usize,
    pub(crate) a: f64,
    pub(crate) w: f64
}

impl Colony{

    pub fn new(
        n : usize , 
        visibility_init: Vec<f64>,
        intensity : f64,
        alfa : f64,
        wisdom : f64,
        evaporation : i128
    )-> Self{
        Self {
            pherohormones :  StableDiGraph::<i128, i128>::new(),
            visibility : visibility_init.clone() ,
            evaporation :   evaporation
            pherohormones_intensity : intensity,
            n_tasks : n,
            a : alfa,
            w : wisdom
        }
    }

    pub fn add_pherohormones(
        &mut self,
        iteration : i128,
        task: usize
    ){ 

        // If there is no adjacent node, create and add pherohormone

        // else just add pherohormone
    }   

    pub fn evaporate_pherormones(&mut self)
    {
      
        //reduce all pherohormones by evaporation rate
    }

}
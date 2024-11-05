use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableDiGraph};
use petgraph::Direction;

pub(crate) struct Colony {
    pub(crate) pherohormones: StableDiGraph<i128, i128>,
    pub(crate) intensity: i128,
    pub(crate) evaporation: i128,
    pub(crate) n_tasks: usize,
}

impl Colony {
    pub fn new(n_tasks: usize, intensity: i128, evaporation: i128) -> Self {
        Self {
            pherohormones: StableDiGraph::new(),
            intensity: intensity,
            evaporation: evaporation,
            n_tasks: n,
        }
    }
    pub fn add_pherohormone(&mut self, source: usize, target: usize) {
        let source = NodeIndex::new(source);
        let target = NodeIndex::new(target);
        if( self.pherohormones.find_edge(source, target).is_some()){
            let edge = self.pherohormones.find_edge(source, target).unwrap();
            let weight = self.pherohormones[edge];
            self.pherohormones.update_edge(edge, weight + self.intensity);
        }else{
            self.pherohormones.add_edge(source, target,  self.intensity);
        }
        
    }
 

}

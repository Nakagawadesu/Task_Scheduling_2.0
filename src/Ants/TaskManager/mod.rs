use petgraph::stable_graph::NodeIndex;

pub(crate) struct TaskTuple {
    pub(crate) node: NodeIndex,
    pub(crate) weight: i128,
}

impl TaskTuple {
    pub fn new(node: NodeIndex, weight: i128) -> Self {
        Self {
            node: node,
            weight: weight,
        }
    }
}

pub(crate) struct TaskManager {
    pub(crate) tasks: Vec<i128>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    // add  task ordered by node number
    pub fn add_task(&mut self, task: i128) {
        for i in 0..self.tasks.len() {
            if self.tasks[i] > task {
                self.tasks.insert(i, task);
                return;
            }
        }
    }
    //linear search
    pub fn remove_task(&mut self, task: i128) {
        let index = self.tasks.iter().position(|&x| x == task).unwrap();
        self.tasks.remove(index);
    }
    // linear search
    pub fn get_task(&mut self, task: i128) -> i128 {
        let index = self.tasks.iter().position(|&x| x == task).unwrap();
        return self.tasks[index];
    }
    pub fn get_tasks(&mut self) -> Vec<i128> {
        return self.tasks.clone();
    }
    pub fn get_size(&mut self) -> i128 {
        return self.tasks.len() as i128;
    }
}

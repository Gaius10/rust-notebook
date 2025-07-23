
use std::collections::HashMap;

pub struct Database {
    data: HashMap<String, Vec<String>>,
}

impl Database {
    pub fn new() -> Self {
        let data: HashMap<String, Vec<String>> = HashMap::new();
        Self { data }
    }

    pub fn list(&self) {
        for (group, employees) in (&self.data).into_iter() {
            println!("Employees at group {group}:");
            for name in employees {
                println!(" - {name}");
            }
        }
    }

    pub fn add(&mut self, employee: String, group: String) {
        let list = self.data.entry(group).or_insert(Vec::new());
        list.push(employee);
    }
}


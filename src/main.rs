use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
	forks: Vec<Mutex<()>>,
}

struct Philosopher {
	name: String,
	left: usize,
	right: usize,
}

impl Philosopher {
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
			name: name.to_string(),
			left: left,
			right: right,
		}
	}
	
	fn eat(&self, table: &Table) {
		let _left = table.forks[self.left].lock().unwrap();
		let _right = table.forks[self.right].lock().unwrap();
	
		println!("{} started eating", self.name);

		let duration = Duration::new(1, 0);
		thread::sleep(duration);
	
		println!("{} finished eating", self.name);
	}
}

fn main() {
	let table = Arc::new(Table { forks: vec![
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
	]});

    let philosophers = vec![
		Philosopher::new("Djudit Buttler", 0, 1),
		Philosopher::new("Raja Khan", 1, 2),
		Philosopher::new("Don Djohnson", 2, 3),
		Philosopher::new("Emmy Roth", 3, 4),
		Philosopher::new("Anna Gold", 0, 4),
	];
	
	let handles: Vec<_> = philosophers.into_iter().map(|p| {
		let table = table.clone();
	
		thread::spawn(move || {
			p.eat(&table);
		})
	}).collect();
	
	for h in handles {
		h.join().unwrap();
	}
}

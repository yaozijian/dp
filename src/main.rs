use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
	let mut g = Graph::with_capacity(6);
	let mut n = Node::new("S");
	g.insert(n.name.clone(), {
		n.to("A", 10).to("B", 20);
		n
	});

	n = Node::new("A");
	g.insert(n.name.clone(), {
		n.to("C", 30).to("D", 10);
		n
	});

	n = Node::new("B");
	g.insert(n.name.clone(), {
		n.to("D", 20);
		n
	});

	n = Node::new("C");
	g.insert(n.name.clone(), {
		n.to("D", 5).to("T", 20);
		n
	});

	n = Node::new("D");
	g.insert(n.name.clone(), {
		n.to("T", 10);
		n
	});

	n = Node::new("T");
	g.insert(n.name.clone(), n);

	shortest_path(&mut g, "S".to_string(), "T".to_string());

	g.reset();
	shortest_path(&mut g, "C".to_string(), "T".to_string());
}

struct Node {
	name: String,
	// 从本点可以到达的点以及路程
	to: HashMap<String, i32>,
	// 从起点到达本点的最短路径长度
	shortest: Cell<i32>,
	// 从起点到达本点的最短路径
	path: RefCell<Vec<String>>,
}

impl Node {
	fn new(n: &str) -> Self {
		Node {
			name: n.to_string(),
			to: HashMap::with_capacity(10),
			shortest: Cell::new(std::i32::MAX),
			path: RefCell::new(Vec::with_capacity(10)),
		}
	}
	fn to(&mut self, d: &str, c: i32) -> &mut Self {
		self.to.insert(d.to_string(), c);
		self
	}
	fn clear(&self) {
		self.shortest.set(std::i32::MAX);
		self.path.borrow_mut().clear();
	}
}

type Graph = HashMap<String, Node>;

trait Reset {
	fn reset(&self);
}

impl Reset for Graph {
	fn reset(&self) {
		for item in self.iter() {
			item.1.clear();
		}
	}
}

fn shortest_path(g: &mut Graph, f: String, t: String) {

	// 从起点到起点的最短路径为零
	g.entry(f.clone()).and_modify(|v| {
		v.shortest.set(0);
		v.path.borrow_mut().push(v.name.clone());
	});

	// 存放下一步要处理的节点
	let mut list: Vec<String> = vec![f.clone()];

	while list.len() > 0 {

		// 取出一个待处理的节点(源节点)
		let name = list.pop().unwrap();
		let src = g.get(&name).unwrap();

		// 对每个可以从源节点到达的节点
		for (k, v) in src.to.iter() {
			// cmp = 起点 -> 源节点 -> 本节点 的路径长度
			let cmp = src.shortest.get() + v;
			let dst = g.get(k).unwrap();
			// 如果 路径长度[起点 -> 源节点 -> 本节点] < 路径长度[起点 -> 本节点]
			// 则替换从起点到本节点的最短路径
			if cmp < dst.shortest.get() {
				dst.shortest.set(cmp);
				let mut path = dst.path.borrow_mut();
				path.clear();
				// 复制路径: 起点 -> 源节点
				for x in src.path.borrow().iter() {
					path.push(x.clone());
				}
				// 加上本节点: 起点 -> 源节点 -> 本节点
				path.push(dst.name.clone());
			}
			// 存放到链表中,等待处理
			list.push(k.clone());
		}
	}

	let dest = g.get(&t).unwrap();
	println!(
		"{} -> {} shortest: {} path: {:?}",
		f, t, dest.shortest.get(),
		dest.path.borrow()
	);
}


use std::cell::RefCell;

fn main() {
	lis(vec![1, 3, 4, 6, 9, 7, 8]);
}

struct LIS {
	lis: RefCell<Vec<i32>>,
}

impl LIS {
	fn new(v: i32) -> Self {
		LIS { lis: RefCell::new(vec![v]) }
	}

	fn len(&self) -> usize {
		self.lis.borrow().len()
	}

	fn append(&self, v: i32) -> Vec<i32> {
		let mut x = self.lis.borrow().clone();
		x.push(v);
		x
	}
}

fn lis(array: Vec<i32>) {
	use std::collections::HashMap;
	// key: 元素下标 => val: 以第key个元素为结束元素的LIS
	let mut lis = HashMap::with_capacity(array.len());

	// 以每个元素为结束元素的LIS，只有一个元素，就是这个元素本身
	for idx in 0..array.len() {
		lis.insert(idx, LIS::new(array[idx]));
	}

	// 对每个元素
	for idx in 0..(array.len() - 1) {
		let cur = lis.get(&idx).unwrap();
		// 对下标大于这个元素的元素
		for p in (idx + 1)..array.len() {
			// 如果元素值也更大
			if array[p] > array[idx] {
				let item = lis.get(&p).unwrap();
				// 并且 lis[idx] + 1 > lis[p],则替换 lis[p]
				if cur.len() + 1 > item.len() {
					item.lis.replace(cur.append(array[p]));
				}
			}
		}
	}

	let mut maxlen = 0usize;
	let mut maxkey = 0usize;

	for (k, v) in lis.iter() {
		if v.len() > maxlen {
			maxlen = v.len();
			maxkey = *k;
		}
	}

	let item = lis.get(&maxkey).unwrap();

	println!("max: {} lis: {:?}", maxlen, item.lis.borrow());
}


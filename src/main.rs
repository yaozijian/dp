fn main() {
    let mut g = Graph::with_capacity(6);
    let mut n = Node::new("S");
    g.insert(n.name.clone(),{n.to("A",10).to("B",20);n});

    n = Node::new("A");
    g.insert(n.name.clone(),{n.to("C",30).to("D",10);n});
    
    n = Node::new("B");
    g.insert(n.name.clone(),{n.to("D",20);n});
    
    n = Node::new("C");
    g.insert(n.name.clone(),{n.to("D",5).to("T",20);n});
    
    n = Node::new("D");
    g.insert(n.name.clone(),{n.to("T",10);n});
    
    n = Node::new("T");
    g.insert(n.name.clone(),n);

    shortest_path(&mut g,"S".to_string(),"T".to_string());
    
    g.reset();
    shortest_path(&mut g,"C".to_string(),"T".to_string());
}

use std::collections::HashMap;
use std::cell::Cell;
use std::cell::RefCell;

struct Node{
    name : String,
    to: HashMap<String,i32>,
    shortest : Cell<i32>,
    path: RefCell<Vec<String>>,
}

impl Node{
    fn new(n:&str) -> Self{
        Node{
            name: n.to_string(),
            to: HashMap::with_capacity(10),
            shortest: Cell::new(std::i32::MAX),
            path: RefCell::new(Vec::with_capacity(10)),
        }
    }
    fn to(&mut self,d:&str,c:i32) -> &mut Self{
        self.to.insert(d.to_string(),c);
        self
    }
    fn clear(&self){
        self.shortest.set(std::i32::MAX);
        self.path.borrow_mut().clear();
    }
}

type Graph = HashMap<String,Node>;
trait Reset{
    fn reset(&self);
}

impl Reset for Graph{
    fn reset(&self){
        for item in self.iter(){
            item.1.clear();
        }
    }
}

fn shortest_path(g : &mut Graph,f:String,t:String){

    g.entry(f.clone()).and_modify(|v|{
        v.shortest.set(0);
        v.path.borrow_mut().push(v.name.clone());
    });

    let mut list : Vec<String>  = vec![f.clone()];

    while list.len() > 0{
        let name = list.pop().unwrap();
        let src = g.get(&name).unwrap();
        for (k,v) in src.to.iter(){
            let cmp = src.shortest.get() + v;
            let dst = g.get(k).unwrap();
            if cmp < dst.shortest.get(){
                dst.shortest.set(cmp);
                let mut path = dst.path.borrow_mut();
                path.clear();
                for x in src.path.borrow().iter(){
                    path.push(x.clone());
                }
                path.push(dst.name.clone());
            }
            list.push(k.clone());
        }
    }

    let dest = g.get(&t).unwrap();
    println!(
        "{} -> {} shortest: {} path: {:?}",
        f,t,dest.shortest.get(),
        dest.path.borrow()
    );
}


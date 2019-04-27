fn main() {
    lis(vec![1,3,4,6,9,7,8]);
}

fn lis(array : Vec<i32>){
    
    use std::collections::HashMap;
    let mut lis = HashMap::with_capacity(array.len());

    for idx in 0..array.len(){
        lis.insert(idx,vec![array[idx]]);
    }

    for idx in (0..array.len()).rev(){
        for p in (0..idx).rev(){
            if array[p] < array[idx]{
                let mut t = lis.get(&p).unwrap().clone();
                lis.entry(idx).and_modify(|v:&mut Vec<i32>|{
                    t.append(v);
                    *v = t
                });
            }
        }
    }

    let mut max = (0usize,vec![]);
    for item in lis.iter(){
        if item.1.len() > max.1.len(){
            max.0 = *item.0;
            max.1 = item.1.clone();
        }
    }

    println!("max: {} lis: {:?}",max.0,max.1);
}


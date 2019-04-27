fn main() {
    find_coins(20);
    find_coins_ex(20);
    find_coins_push(20);
    find_coins_push_ex(20);
}

fn find_coins(target : usize){

    let target = target + 1usize;
    let mut coins = Vec::with_capacity(target);
    coins.resize(target,0usize);

    for idx in 1..target{
        let mut cost = target;
        if idx >= 1  { cost = std::cmp::min(cost,1 + coins[idx - 1]); }
        if idx >= 5  { cost = std::cmp::min(cost,1 + coins[idx - 5]); }
        if idx >= 11 { cost = std::cmp::min(cost,1 + coins[idx - 11]);}
        coins[idx] = cost;
        println!("f[{}]={}",idx,cost);
    }
}

fn find_coins_ex(target : usize){

    let target = target + 1usize;
    let mut coins : Vec<Vec<i32>> = Vec::with_capacity(target);
    coins.resize(target,Vec::with_capacity(target));

    let select = |coins:&Vec<Vec<i32>>,ctx:&mut (i32,i32,i32),face:i32|{
        if ctx.1 >= face{
            let newcost = 1 + coins[(ctx.1 - face) as usize].len();
            let newcost = newcost as i32;
            if newcost < ctx.0{
                ctx.0 = newcost;
                ctx.2 = face;
            }
        }
    };

    for idx in 1..target{
        // 0: cost 1: target 2: face
        let mut choice = (target as i32,idx as i32,0i32);
        select(&coins,&mut choice,1);
        select(&coins,&mut choice,5);
        select(&coins,&mut choice,11);
        
        coins[idx] = coins[idx - choice.2 as usize].clone();
        coins[idx].push(choice.2);
        
        assert_eq!(choice.0 as usize,coins[idx].len());
        
        println!("f[{}]={}: {:?}",idx,choice.0,coins[idx]);
    }
}

fn find_coins_push(target : i32){

    use std::collections::HashMap;

    let mut f = HashMap::with_capacity(target as usize + 1); 
    let mut t1 = Vec::with_capacity(target as usize * 10);

    let mut add = |(val,cost),face,t2:&mut Vec<(i32,i32)>|{
        
        let v = val + face;
        let c = cost + 1;
        let mut m = v <= target;

        if m{
            f.entry(v)
                .and_modify(|v|{ m = c < *v; if m { *v = c; }}) 
                .or_insert(c);
        }
        if m{
            t2.push((v,c));
        }
    };

    t1.push((0,0));

    while t1.len() > 0{
        let mut t2 = Vec::with_capacity(target as usize * 10);
        for item in t1.into_iter(){
            add(item,1,&mut t2);
            add(item,5,&mut t2);
            add(item,11,&mut t2);
        }
        t1 = t2;
    }

    for idx in 1..target+1{
        println!("{}: {:?}",idx,f.get(&idx));
    }
}

fn find_coins_push_ex(target : i32){

    use std::collections::HashMap;

    let mut f = HashMap::with_capacity(target as usize + 1); 
    let mut t1 = Vec::with_capacity(target as usize * 10);
    
    let clone = |c:&Vec<i32>,f:i32|{
        let mut x = c.clone();
        x.push(f);
        x
    };

    let mut add = |
        (val,cost):&(i32,Vec<i32>),
        face:i32,
        t2:&mut Vec<(i32,Vec<i32>)>|{
        
        let v = val + face;
        let c = cost.len() + 1;
        let mut m = v <= target;

        if m{
            f.entry(v)
                .and_modify(|v:&mut Vec<i32>|{ 
                    m = c < v.len(); 
                    if m {
                        *v = clone(&cost,face);
                    }
                }) 
                .or_insert(clone(&cost,face));
        }
        if m{
            t2.push((v,clone(&cost,face)));
        }
    };

    t1.push((0,vec![] as Vec<i32>));

    while t1.len() > 0{
        let mut t2 = Vec::with_capacity(target as usize * 10);
        for item in t1.into_iter(){
            add(&item,1,&mut t2);
            add(&item,5,&mut t2);
            add(&item,11,&mut t2);
        }
        t1 = t2;
    }

    for idx in 1..target+1{
        println!("{}: {:?}",idx,f.get(&idx));
    }
}


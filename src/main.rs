fn main() {
    find_coins(20);
    find_coins_ex(20);
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


fn main() {
    find_coins(20);
    find_coins_ex(20);
    find_coins_push(20);
    find_coins_push_ex(20);
}

fn find_coins(target: usize) {
    let target = target + 1usize;
    let mut coins = Vec::with_capacity(target);
    coins.resize(target, 0usize);

    for idx in 1..target {
        let mut cost = target;
        // 分别选择三种面额的钞票: 开销 = 1 (选择的这张钞票) + 剩余目标值的开销
        if idx >= 1 { cost = std::cmp::min(cost, 1 + coins[idx - 1]); }
        if idx >= 5 { cost = std::cmp::min(cost, 1 + coins[idx - 5]); }
        if idx >= 11 { cost = std::cmp::min(cost, 1 + coins[idx - 11]); }
        coins[idx] = cost;
        println!("f[{}]={}", idx, cost);
    }
}

fn find_coins_ex(target: usize) {
    let target = target + 1usize;
    // 下标: 目标值 元素值: 凑出目标值使用的钞票面额序列
    let mut coins: Vec<Vec<i32>> = Vec::with_capacity(target);
    coins.resize(target, Vec::with_capacity(target));

    let select = |coins: &Vec<Vec<i32>>, ctx: &mut (i32, i32, i32), face: i32| {
        if ctx.1 >= face {
            // 目标值 不小于 面额: 则 开销 = 1(选择的面值为face的钞票) + 剩余目标值的开销
            let newcost = 1 + coins[(ctx.1 - face) as usize].len();
            let newcost = newcost as i32;
            // 开销更小则选择这张面额的钞票
            if newcost < ctx.0 {
                ctx.0 = newcost;
                ctx.2 = face;
            }
        }
    };

    for idx in 1..target {
        // 0: cost 1: target 2: face
        let mut choice = (target as i32, idx as i32, 0i32);
        // 分别尝试选择三种面额的钞票
        select(&coins, &mut choice, 1);
        select(&coins, &mut choice, 5);
        select(&coins, &mut choice, 11);

        // 使用的钞票序列 = 凑出剩余目标值使用的钞票序列 + 本次选择的钞票
        coins[idx] = coins[idx - choice.2 as usize].clone();
        coins[idx].push(choice.2);

        // 开销 = 钞票序列的长度
        assert_eq!(choice.0 as usize, coins[idx].len());

        println!("f[{}]={}: {:?}", idx, choice.0, coins[idx]);
    }
}

fn find_coins_push(target: i32) {
    use std::collections::HashMap;

    // key = 目标值 => val = 开销
    let mut f = HashMap::with_capacity(target as usize + 1);
    let mut t1 = Vec::with_capacity(target as usize * 10);

    let mut add = |(val, cost), face, t2: &mut Vec<(i32, i32)>| {
        // val: 当前目标值 cost: 当前开销
        // face: 选择的钞票面额
        let v = val + face;
        let c = cost + 1;
        let mut m = v <= target;// 不处理太大的目标值

        if m {
            f.entry(v)
             .and_modify(|cur| {
                 m = c < *cur;
                 if m { *cur = c; }// 如果凑得目标值 v 的开销更小,则选择当前钞票面额
             })
             .or_insert(c);
        }

        // 如果选择面额为face的钞票,则记录下来: 目标值=v,开销=c
        if m {
            t2.push((v, c));
        }
    };

    t1.push((0, 0));

    while t1.len() > 0 {
        let mut t2 = Vec::with_capacity(target as usize * 10);
        // 对每个原有状态,分别取三种面额的钞票,记录凑得目标值的最小开销到t2中
        for item in t1.into_iter() {
            add(item, 1, &mut t2);
            add(item, 5, &mut t2);
            add(item, 11, &mut t2);
        }
        t1 = t2;
    }

    for idx in 1..target + 1 {
        println!("{}: {:?}", idx, f.get(&idx));
    }
}

fn find_coins_push_ex(target: i32) {
    use std::collections::HashMap;

    // key = 目标值 => val = 凑得目标值的最少钞票序列(开销=序列长度)
    let mut f = HashMap::with_capacity(target as usize + 1);
    let mut t1 = Vec::with_capacity(target as usize * 10);

    // 在已有钞票序列c的基础上,增加选择面额为f的钞票
    let clone = |c: &Vec<i32>, f: i32| {
        let mut x = c.clone();
        x.push(f);
        x
    };

    // val: 目标值 cost: 凑得目标值的最少钞票序列
    // face: 尝试使用的钞票面额
    // t2: 如果选择了尝试使用的钞票,则记录到t2中
    let mut add = |(val, cost): &(i32, Vec<i32>),
                   face: i32,
                   t2: &mut Vec<(i32, Vec<i32>)>| {
        let v = val + face;
        let c = cost.len() + 1;
        let mut m = v <= target;// 不处理太大的目标值

        if m {
            f.entry(v)
             .and_modify(|cur: &mut Vec<i32>| {
                 m = c < cur.len();
                 if m {
                     // 如果凑得目标值 v 的开销更小(新开销c < 当前开销cur.len()),则选择面额为face的钞票
                     *cur = clone(&cost, face);
                 }
             })
             .or_insert(clone(&cost, face));
        }

        // 如果选择面额为face的钞票,则记录下来: 目标值=v,钞票序列=原有序列 + 新选择的面额为face的钞票
        if m {
            t2.push((v, clone(&cost, face)));
        }
    };

    t1.push((0, vec![] as Vec<i32>));

    while t1.len() > 0 {
        let mut t2 = Vec::with_capacity(target as usize * 10);
        // 对每个原有状态,分别取三种面额的钞票,记录凑得目标值的最小开销到t2中
        for item in t1.into_iter() {
            add(&item, 1, &mut t2);
            add(&item, 5, &mut t2);
            add(&item, 11, &mut t2);
        }
        t1 = t2;
    }

    for idx in 1..target + 1 {
        println!("{}: {:?}", idx, f.get(&idx));
    }
}


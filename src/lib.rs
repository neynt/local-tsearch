use std::fmt::Display;

pub struct LocalSearchConfig<'a, T: 'a + Display> {
    pub start_state: &'a Fn() -> T,
    pub cost: &'a for<'r> Fn(&'r T) -> f64,
    pub neighbor: &'a (for<'r> Fn(&'r T) -> T),
    pub num_restarts: usize,
}

pub fn local_search<T: Display>(c: &LocalSearchConfig<T>) -> T {
    let mut best: T = (c.start_state)();
    let mut best_cost: f64 = (c.cost)(&best);
    println!("Init: {}", best);
    println!("Cost: {}", best_cost);
    for _restart_idx in 0 .. c.num_restarts {
        let mut cur = (c.start_state)();
        let mut cur_cost = (c.cost)(&cur);
        // current strategy is very basic.
        // keep generating neighbors and instantly take improving neighbors.
        // stop when 100 neighbors in a row fail to improve.
        'improvements: loop {
            for _attempt in 0 .. 100 {
                let neighbor = (c.neighbor)(&cur);
                let neighbor_cost = (c.cost)(&neighbor);
                if neighbor_cost < cur_cost {
                    cur = neighbor;
                    cur_cost = neighbor_cost;
                    continue 'improvements;
                }
            }
            break;
        }
        if cur_cost < best_cost {
            best = cur;
            best_cost = cur_cost;
            println!("Best: {}", best);
            println!("Cost: {}", best_cost);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

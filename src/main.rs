use itertools::Itertools;
use diam::prelude::*;
use diam::join;
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;
use std::cmp;
use rand::Rng;

fn seq_max_profit(prices: Vec<i32>) -> i32 {
    prices.into_iter()
    .fold((std::i32::MAX, 0), |(low, profit), price|
        (
            cmp::min(low, price),
            cmp::max(profit, price - low)
        )
    )
    .1
}

struct Stock {
    low: i32,
    high: i32,
    max_profit: i32,
}

fn par_max_profit(start_index: usize, end_index: usize, prices: Vec<i32>) -> Stock {
    if end_index - start_index <= 1 {
        return Stock {
                low: cmp::min(prices[start_index].clone(), prices[end_index].clone()),
                high: cmp::max(prices[start_index].clone(), prices[end_index].clone()),
                max_profit: 0,
            }
    }

    let mid = (start_index + end_index + 1) / 2;

    println!("Mid is {:?}", mid);

    let (left_stock, right_stock) = join(
        || par_max_profit(start_index, mid, prices.clone()),
        || par_max_profit(mid + 1, end_index, prices.clone()),
    );

    let local_low = cmp::min(left_stock.low, right_stock.low);
    let local_high = cmp::max(left_stock.high, right_stock.high);
    let local_max_profit = cmp::max(
                                        cmp::max(left_stock.max_profit, right_stock.max_profit),
                                        right_stock.high - left_stock.low
                                    );
    
    // Returns (lowest low, highest high, local max profit)
    Stock {
        low: local_low,
        high: local_high,
        max_profit: local_max_profit
    }
}

const N: usize = 10_000;
fn main() {
    let mut rng = rand::thread_rng();

    let input: Vec<i32> = (0..N).map(|_| rng.gen_range(0..100_000_000)).collect();
    let end_index = input.len() - 1;

    println!("bom dia {:?}", input);

    let seq_result = seq_max_profit(input.clone());
    let par_result = par_max_profit(0, end_index, input.clone());

    println!("Max profit is {:?} x {:?}", seq_result, par_result.max_profit);
}

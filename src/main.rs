use diam::join;
use std::cmp;
use rand::Rng;


struct Stock {
    low: i32,
    high: i32,
    max_profit: i32,
}

fn helper_seq_max_max_profit(prices: &[i32]) -> Stock {
    let (low, high, max_profit) = prices.into_iter()
    .fold((std::i32::MAX, std::i32::MIN, 0), |(low, high, max_profit), cur_price|
        (
            cmp::min(low, *cur_price),
            cmp::max(high, *cur_price),
            cmp::max(max_profit, *cur_price - low)
        )
    );
    Stock {
        low: low,
        high: high,
        max_profit: max_profit,
    }
}

fn par_max_profit(start_index: usize, end_index: usize, prices: &Vec<i32>) -> Stock {
    if end_index - start_index <= 1000 {
        return helper_seq_max_max_profit(&prices[start_index..end_index]);
    }
    
    let mid = (start_index + end_index + 1) / 2;
    let (left_stock, right_stock) = join(
        || par_max_profit(start_index, mid, prices),
        || par_max_profit(mid + 1, end_index, prices),
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

fn seq_max_profit(prices: Vec<i32>) -> i32 {
    prices.into_iter()
    .fold((std::i32::MAX, 0), |(low, max_profit), cur_price|
        (
            cmp::min(low, cur_price),
            cmp::max(max_profit, cur_price - low)
        )
    )
    .1
}

const N: usize = 100_000;
fn main() {
    let mut rng = rand::thread_rng();

    let input: Vec<i32> = (0..N).map(|_| rng.gen_range(0..900_000_000)).collect();
    let end_index = input.len() - 1;

    let start = std::time::Instant::now();
    let seq_result = seq_max_profit(input.clone());
    println!("Sequential result is: {:?}, took {:?}", seq_result, start.elapsed());

    let start = std::time::Instant::now();
    let par_result = par_max_profit(0, end_index, &input.clone());
    println!("Parallel result is: {:?}, took {:?}", par_result.max_profit, start.elapsed());
    // diam::svg("parallel-version.svg", || {
    //     par_max_profit(0, end_index, &input.clone());
    // });
}

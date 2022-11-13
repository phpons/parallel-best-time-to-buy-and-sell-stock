# Parallel version of Leetcode problem "Best Time to Buy and Sell Stock"
## The problem

The input of the problem is an array containing the prices of a stock over time. The goal is to find the maximum possible profit given these prices. This requires knowing when is the best time to buy and sell the stock.

### Example
*Input: [7,1,5,3,6,4]*,
*Output: 5*

Explanation: the stock can be bought for 1 and sold for 6, resulting in a profit of 5.

## Sequential algorithm evaluation:
Explanation of the solution:
> Go over each element, storing the minimum element found so far and updating the maximum profit so far whenever a new maximum is found. 
<br>
<br>
This guarantees that the lowest low (so far) is always "matched" with the highest possible value in the stock timeline.

Each element has to be checked exactly once to reach the solution with this algorithm. There is no point where it can be interrupted.

### Rust implementation
The code seen in the image below (and included in this repository) shows the sequential implementation of the solution in Rust.

![Sequential implementation in Rust](images/rust_seq_implementation.png "Rust sequential implementation")

### Complexity
As each element of the array is accessed only once, the complexity of the solution is polynomial 𝒪(n).

------------------
## Parallel solution:
The principle behind the parallel solution is the same as the sequential. The solution has to be adjusted to allow for parallel execution, however: 

This is achieved via continuously splitting the Vector in half and then joining the local results found by the algorithm.

*Note:  the vector is not manipulated in the implementation, the split happens by tracking the start and end indices for each subdivision as function parameters.*

For each split (left_vec, right_vec) in the vector, a struct containing (minElement, maxElement, maxLocalProfit) is returned. They can then be joined in a reduce fashion to obtain the full results:
- minElement = min(minLeftElement, minLeftElement)
- maxElement = max(maxLeftElement, maxLeftElement)
- maxProfit = max(maxLeftProfit, maxRightProfit, maxRightElement - minLeftElement)

### Implementation in Rust
For this problem, the joining operations are not associative, so I opted for using Diam join() rather than fold() and reduce(), as the splitting done by *par_iter()* is non-deterministic and would lead to a more complex implementation.

The code for the parallel implementation can be found in the function *par_max_profit* in the *main.rs* file. It can also be seen in the image below:
![Sequential parallel in Rust](images/rust_par_implementation.png "Rust parallel implementation")
### Complexity
TBD. Still 𝒪(n) anyway.

# Results
## Testing
The parallel implementation was tested against the sequential implementation using randomly generated inputs of size = 100 million.

## Numbers

The results of a few test runs with can be seen below:
| Input size  | Sequential execution | Parallel execution |
|-------------|----------------------|--------------------|
| 100_000     | 225.9µs              | 2.2941ms           |
| 1_000_000   | 1.6144ms             | 18.0495ms          |
| 100_000_000 | 170.2032ms           | 1.5885019s         |
*Note: all runs were done using cargo run --release.*


The parallel implementation was consistently slower than the sequential one in the tests performed. Initially, I thought that might be due to the usage of *clone()* in the parallel function.

After removing all uses of *clone()* inside the function, the performance was improved by around 1000x, which was still not enough to beat the sequential performance.

From these results, I assume the overhead from joining and splitting could explain the worse performance.


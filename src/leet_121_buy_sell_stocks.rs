/*
You are given an array prices where prices[i] is the price of a given stock on the ith day.
You want to maximize your profit by choosing a single day to buy one stock
and choosing a different day in the future to sell that stock.
Return the maximum profit you can achieve from this transaction.

If you cannot achieve any profit, return 0.
*/

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    // get first max by comparing delta between first two
    let mut max: i32 = 0.max(prices[1] - prices[0]);

    // set inital min value
    let mut min: i32 = prices[0];
    println!("start: max: {max}, min: {min}");

    (1..prices.len()).for_each(|i| {
        max = max.max(prices[i] - min);
        min = min.min(prices[i]);
        //println!("debug: max: {max}, min{min}");
    });
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let prices = vec![7, 1, 5, 3, 6, 4];

        assert_eq!(max_profit(prices), 5);
    }
    #[test]
    fn example2() {
        let prices = vec![7, 6, 4, 3, 1];

        assert_eq!(max_profit(prices), 0);
    }
}

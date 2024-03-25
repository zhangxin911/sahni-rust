use std::collections::HashMap;

pub fn factorial(n: i32) -> i32 {
    match (n < 1) {
        true => 1,
        false => n * factorial(n - 1),
    }
}

pub fn sum(arr: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..arr.len() {
        result += arr[i];
    }
    result
}

pub fn r_sum(arr: &[i32], n: i32) -> i32 {
    match (n > 0) {
        true => r_sum(arr, n - 1) + arr[(n - 1) as usize],
        false => 0,
    }
}

pub fn fib(n: i32) -> i32 {
    if (n == 0) {
        return 0;
    }
    if (n == 1) {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
pub fn fib_cache(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = if (n <= 1) {
        n
    } else {
        fib_cache(n - 1, memo) + fib_cache(n - 2, memo)
    };
    memo.insert(n, result);
    result
}

pub fn odd_even_fn(n: i32) -> i32 {
    if (n % 2 == 0) {
        return n / 2;
    } else {
        return odd_even_fn(3 * n + 1);
    }
}

pub fn ackermann_fn(i: i32, j: i32) -> i32 {
    if (i == 1 && j >= 1) {
        return 2_i32.pow(j as u32);
    } else if (i >= 2 && j == 1) {
        return ackermann_fn(i - 1, 2);
    } else {
        return ackermann_fn(i - 1, ackermann_fn(i, j - 1));
    }
}

pub fn gcd<T>(x: T, y: T) -> T
where
    T: std::cmp::PartialEq<i32> + std::ops::Rem<Output = T> + Copy,
{
    if (y == 0) {
        x
    } else {
        gcd(y, x % y)
    }
}

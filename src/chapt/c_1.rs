use std::{
    fmt::Debug,
    ops::{self, Deref},
    process::Output,
};

pub fn abc(a: i32, b: i32, c: i32) -> i32 {
    a + b * c
}

pub fn templateABC<T>(a: T, b: T, c: T) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    a + b * c
}

pub fn refABC<T>(a: &T, b: &T, c: &T) -> T
where
    // &T: Mul<&T>
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    *a + *b * *c
}

// pub fn abcRef<'a, T>(a: &'a T, b: &'a T, c: &'a T) -> &'a T
// where
//     T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
// {
//     a + b * c
// }
pub fn swap<'a>(mut a: &'a i32, mut b: &'a i32) {
    let temp = a;
    a = b;
    b = temp;
    println!("{:?}", a);
}

pub fn swap_mut(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

pub fn array_count(arr: &[i32], value: i32) -> i32 {
    // let arr =
    // let arr: [i32; 5] = [0; 5];
    let mut count = 0;
    for element in arr {
        match *element == value {
            true => count = count + 1,
            false => (),
        }
        println!("{:?}", element);
    }
    println!("{:?}", arr.len());
    count
}

pub fn array_fn_count(arr: &[i32], value: i32) -> usize {
    arr.iter().filter(|&el| *el == value).count()
}

pub fn fill_arry(start: i32, end: i32, arr: &mut [i32], value: i32) {
    if start < end && end < arr.len().try_into().unwrap() {
        for i in start..end {
            arr[i as usize] = value;
        }
    } else {
        println!("无效范围!");
    }
}

pub fn itoa<T>(n: usize, value: T, arr: &mut [T])
where
    T: std::ops::Add<Output = T> + std::convert::From<usize> + Copy,
{
    for index in 0..n {
        arr[index] = value + T::from(index);
    }
}

pub fn itoa1<T>(value: T, arr: &mut [T])
where
    T: std::ops::Add<Output = T> + Copy + std::convert::From<usize>,
{
    for (index, ele) in arr.iter_mut().enumerate() {
        // *ele = value + index as T;
        *ele = value + T::from(index);
    }
}

pub fn is_sorted(arr: &[i32]) -> bool {
    let mut result = true;
    for i in 0..(arr.len() - 1) {
        println!("{:?}", arr[i]);
        if arr[i + 1] - arr[i] < 0 {
            result = false;
            break;
        }
    }
    result
}

pub fn is_sorted_tmp<T>(arr: &[T]) -> bool
where
    T: std::ops::Sub<Output = T> + Debug + Copy + std::cmp::PartialOrd<i32>,
{
    let mut result = true;
    for i in 0..(arr.len() - 1) {
        println!("{:?}", arr[i]);
        if arr[i + 1] - arr[i] < 0 {
            result = false;
            break;
        }
    }
    result
}

pub fn new_example() {
    let x = 5;
    let y = Box::new(x);

    struct MyBox<T> {
        value: T,
    };
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox { value: x }
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    let my_box = MyBox::new(10);

    // let my_arr: Vec<i32> = Vec::new();
    println!("{:?}", *my_box);
}

pub fn martix_arr() {
    let martix_arr = [[1; 5]; 10];
    for row in 0..10 {
        for col in 0..5 {
            println!("{:?}", martix_arr[row][col]);
        }
    }
}

// pub fn martix_arr_vec() {
//     let mut martix_arr_vec: Vec<Vec<i32>> = Vec::new();
//     for i in 0..3 {
//         let mut row: Vec<i32> = Vec::new();
//         for j in 0..2 {
//             row.push(i + j);
//             // let col:Vec<i32> = Vec::new();
//         }
//         martix_arr_vec.push(row);
//     }
// }

pub fn martix_arr_vec() {
    let mut martix_arr_vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..9 {
        let mut row = Vec::new();
        for j in 0..3 {
            row.push(j + i);
        }
        martix_arr_vec.push(row);
    }
}

pub fn make_2d_array<T: Default + Copy>(row: usize, col: usize) -> [[T; 4]; 3] {
    let mut martix = [[T::default(); 4]; 3];
    for i in 0..row {
        for j in 0..col {
            martix[i][j] = T::default();
        }
    }
    martix
}

pub fn make_2d_array_vec<T: Default + Clone>(row: usize, col: usize) -> Vec<Vec<T>> {
    let mut martix = Vec::with_capacity(row);
    for _ in 0..col {
        let row = vec![T::default(); col];
        martix.push(row);
    }
    martix
}

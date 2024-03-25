pub mod chapt;
// use crate::chapt::c_1::abc;
use std::io;

fn main() {
    println!("{:?}", chapt::c_1::abc(1, 2, 3));
    println!("{:?}", chapt::c_1::templateABC(1.1, 2.1, 3.1));
    let a = 1;
    let b = 2;
    let c = 3;
    println!("{:?}", chapt::c_1::refABC(&a, &b, &c));

    let num1 = 1;
    let num2 = 2;
    chapt::c_1::swap(&num1, &num2);
    println!("num1: {:?}, num2: {:?}", num1, num2);
    let mut x = 2;
    let mut y = 4;
    chapt::c_1::swap_mut(&mut x, &mut y);
    println!("x: {:?}, y: {:?}", x, y);
    println!("count: {:?}", chapt::c_1::array_count(&[1, 2, 2, 5], 2));

    let mut arr1 = [0; 10];
    let start = 2;
    let end = 6;
    let fill_value = 99;
    chapt::c_1::fill_arry(start, end, &mut arr1, fill_value);
    println!("{:?}", arr1);

    let arr_sort = [1, 2, 22, 4, 5];
    chapt::c_1::is_sorted(&arr_sort);
    println!("{}", chapt::c_1::is_sorted_tmp(&arr_sort));
    chapt::c_1::new_example();

    // chapt::c_1::martix_arr();

    let martix_2d_arr: [[i32; 4]; 3] = chapt::c_1::make_2d_array(2, 3);
    println!("{:?}", martix_2d_arr);
    // chapt::c_1::change_length_1d(array, new_len)
    chapt::c_1::test_change_length_1d();

    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            println!("{:?}", buf);
        }
        Err(err) => {
            println!("get input error {:?}", err);
        }
    }

    let factorial_num = 5;
    println!(
        "factorial result is: {:?} ",
        chapt::c_1_7::factorial(factorial_num)
    );
    let sum_arr = [3, 2, 3, 2, 3, 2];
    println!("sum of sum_arr is {:?}", chapt::c_1_7::sum(&sum_arr));

    println!("fib result is {:?}", chapt::c_1_7::fib(4
    ));
}

// fn main() {
//     let number = 6;
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }

//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is {}", number);
// }

// loop: Бесконечный цикл. Выход из него осуществляется с помощью break. break также может возвращать значение из цикла.
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {}", result);
// }
// while: Цикл выполняется, пока условие истинно.
// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//     }
//     println!("Start!");
// }
// for: Самый распространенный цикл, используется для итерации по коллекции (например, массиву, вектору или диапазону).
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Start!");
}

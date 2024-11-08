use std::ops::{Range, RangeInclusive};

#[test]
fn test411() {
    let x: i32 = 5;
    let mut y: i32 = 5; // Змінюємо тип y на i32

    y = x; // Тепер присвоєння працюватиме

    let z = 10; // Тип змінної z виводиться автоматично як i32

    println!("Success!");
}


#[test]
fn test412() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");

}


#[test]
fn test43() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");

    // Get the type of given variable, return a string representation of the type, e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

#[test]
fn test44() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}


#[test]
fn test45() {
    let v1 = 251_u8.checked_add(8).unwrap_or_else(|| {
        println!("Overflow occurred in v1 calculation");
        0
    });

    let v2 = i8::checked_add(120, 8).unwrap_or_else(|| {
        println!("Overflow occurred in v2 calculation");
        0
    });

    println!("{},{}", v1, v2);
}


#[test]
fn test416() {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        assert!(v == 1597);

        println!("Success!");


}

#[test]
fn test417() {
    {
        let x = 1_000.000_1; // f64 за замовчуванням
        let y: f32 = 0.12; // f32
        let z = 0.01_f64; // f64

        assert_eq!(type_of(&x), "f64".to_string());
        println!("Success!");
    }
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}


#[test]
fn test418() {
        let epsilon = 1e-10_f64;
        assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < epsilon);

        println!("Success!");
    }



#[test]
fn test419() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

#[test]
fn test410() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

#[test]
fn test4111() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);

    assert!(3 * 50 == 150);

    // Floating-point division with explicit f64 type
    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < 1e-10);

    assert!(24 % 5 == 4);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // Результат: 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);  // Результат: 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // Результат: 0110
    println!("1 << 5 is {}", 1u32 << 5);                    // Результат: 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);          // Результат: 0x20
}
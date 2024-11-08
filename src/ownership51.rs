#[test]
fn test511() {
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}

#[test]
fn test512() {
    {
        let s1 = String::from("Hello world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s // Повертаємо значення `s`, щоб передати його назад у main

    }
}

#[test]
fn test513() {
    {
        let s = give_ownership();
        println!("{}", s);
    }

    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("Hello world");
        s // Просто повертаємо `s`, не викликаючи `.into_bytes()`
    }
}

#[test]
fn test514() {
    {
        let s = String::from("Hello World");

        print_str(&s); // Передаємо `s` як посилання

        println!("{}", s); // `s` все ще доступна тут
    }

    fn print_str(s: &String) { // Приймаємо `s` як посилання
        println!("{}", s);
    }
}

#[test]
fn test515() {
    let x = (1, 2, (), "hello"); // Змінюємо "hello".to_string() на "hello" (&str)
    let y = x; // Тепер `x` реалізує Copy і може бути скопійований автоматично
    println!("{:?}, {:?}", x, y);
}

#[test]
fn test516() {
    let mut s1 = String::from("Hello "); // Оголошуємо `s1` як змінну змінну

    s1.push_str("World!");

    println!("Success!");
}

#[test]
fn test517() {
    let mut x = Box::new(5); // Оголошуємо `x` як mutable

    let y = &mut *x; // Створюємо змінне посилання `y` на значення всередині `Box`

    *y = 4;

    assert_eq!(*x, 4); // Перевіряємо значення `x`, яке тепер дорівнює 4

    println!("Success!");
}


#[test]
fn test518() {
    let t = (String::from("hello"), String::from("world"));

    let _s = &t.0; // Беремо посилання на t.0, замість того щоб переміщувати значення

    // Тепер t залишається повністю доступним
    println!("{:?}", t);
}


#[test]
fn test519() {
    let t = (String::from("hello"), String::from("world"));

    // Деструктуруємо кортеж з посиланнями на його елементи
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
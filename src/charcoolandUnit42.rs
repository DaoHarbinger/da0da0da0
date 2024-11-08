use std::mem::size_of_val;
#[test]
fn test421() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // Розмір `char` у Rust завжди 4 байти

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); // Розмір `char` у Rust завжди 4 байти

    println!("Success!");
}

#[test]
fn test422() {
    {
        let c1 = "中".chars().next().unwrap();
        print_char(c1);
    }

    fn print_char(c: char) {
        println!("{}", c);
    }
}

#[test]
fn test423(){
    let _f: bool = false;

    let t = false; // змінюємо значення на false
    if !t {
        println!("Success!");
    }
}


#[test]
fn test424() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test425() {
    {
        let _v: () = ();

        let v = (); // Змінюємо v на тип ()
        assert_eq!(v, implicitly_ret_unit());

        println!("Success!");
    }

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }
}


#[test]
fn test426() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 0); // Розмір типу `()` дорівнює 0 байт

        println!("Success!");
    }


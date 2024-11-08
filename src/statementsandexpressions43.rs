

#[test]
fn test430() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

#[test]
fn test431() {
    let v = {
        let mut x = 1;
        x += 2;
        x // Явне повернення значення `x`
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]
fn test432() {
    let v = {
        let x = 3;
        x // Повертаємо значення `x` як останній вираз блоку
    };

    assert!(v == 3);

    println!("Success!");
}

#[test]
fn test433() {
    {
        let s = sum(1, 2);
        assert_eq!(s, 3);

        println!("Success!");
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y // Видаляємо крапку з комою, щоб повернути результат
    }
}
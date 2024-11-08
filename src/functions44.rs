

#[test]
fn test441() {
    {
        // Don't modify the following two lines!
        let (x, y) = (1, 2);
        let s = sum(x, y);

        assert_eq!(s, 3);

        println!("Success!");
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y // Видаляємо крапку з комою, щоб повернути результат
    }
}

#[test]
fn test442() {
    {
        print();
    }

    // Змінюємо тип повернення на ()
    fn print() -> () {
        println!("Success!");
    }
}

#[test]
fn test443() {
    {
        never_return();
        println!("Failed!");
    }

    fn never_return() -> ! {
        loop {
            // Цей нескінченний цикл ніколи не завершиться
        }
    }
}

#[test]
fn test444() {
    fn never_return_fn() -> ! {
        loop {
            // Цей нескінченний цикл не дозволить функції завершитися
        }
    }

}

#[test]
fn test445() {
    let b = false;

    let _v = match b {
        true => 1,
        // Використовуємо нескінченний цикл замість panic!, щоб програма не завершувалася і не виводила помилку
        false => {
            println!("Success!");
            loop {
                // Нескінченний цикл не дозволить програмі завершитися
            }
        }
    };

    println!("Exercise Failed if printing out this line!");
}

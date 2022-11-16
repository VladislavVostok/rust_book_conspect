/*
    6.      Перечисления и сопоставление с образцом (Enums and Pattern Matching)
    6.2.    Управляющая конструкция match (The match Control Flow Construct)

    /*
        Механихм управления потоком match позволяет сравнивать значения шаблона и затем выполнять код, аналогией может являться конструктор ветвления if. 
        Шаблоны могут состоять из литеральныъ значений, имён переменных, подстановочныъ знаков и многое другое. Отлично сочетается с перечислениями.
        Сила match заключается в вырадительности шаблонов и в том, чо компилятор проверяет, что все возможные случаи обработаны.
    */
*/


fn main() {
    /* Первая версия */
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents_2(Coin::Penny));

    /* Шаблоны, которые привязывают значения */

    // Есть ещё одно полезное качество у веток в выражении match: 
    // они могут привязываться к частям тех значений, которые совпали с шаблоном. 
    // Благодаря этому можно извлекать значения из вариантов перечисления.
    println!("{}", value_in_cents_3(Coin2::Quarter(UsState::Alaska)));

    /* 
        Если мы сделаем вызов функции value_in_cents_3(Coin2::Quarter(UsState::Alaska)), 
        то coin будет иметь значение Coin2::Quarter(UsState::Alaska). Когда мы будем 
        сравнивать это значение с каждой из веток, ни одна из них не будет совпадать, 
        пока мы не достигнем варианта Coin2::Quarter(state). В этот момент state привяжется к 
        значению UsState::Alaska. Затем мы сможем использовать эту привязку в выражении println!, 
        получив таким образом внутреннее значение варианта Quarter перечисления Coin2. 
    */

}

/* Первая версия */
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Если кода для обработки болеше, нужны фигурные скобки.

fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/* Шаблоны, которые привязывают значения */
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabaam,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_3(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
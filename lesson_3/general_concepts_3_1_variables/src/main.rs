/*
    3.      Общие концепции программирования (Common Programming Concepts)
    3.1.    Переменные и понятие изменяемости (Variables and Mutability)
*/

// 1. Переменные
// По-умолчанию все переменные в Rust неизменяемые
// 2. Константы
// Всегда нужно явно указывать тип константы
// 3. Затенение переменной


fn main() {
    
    // 1. Переменные
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;   // Ошибка если перед переменной x нет ключевого слова mut ^^^^^ cannot assign twice to immutable variable
    println!("The value of x is: {x}");

    // 2. Константы
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    // 3. Затенение переменной
    let x = 5;
    let x = x + 1;
    {   // Создание внутренней области видимости
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Отличие между mut и затенением, при слове mut мы можем изменять значение одного и тогоже типа.
    // При затенении мы можем переиспользвать имя переменной и расположить туда значение другого типа.

    let spaces = "    ";
    let spaces = spaces.len();

    // let mut spaces = "    ";
    // spaces = spaces.let();   // ^^^^^^^^^^^^ expected `&str`, found `usize`

}

/*
    3.      Общие концепции программирования (Common Programming Concepts)
    3.5.    Управляющие конструкции (Control Flow)
*/


fn main() {

    // УПРАВЛЯЮЩИЕ КОНСТРУКЦИИ
    // Выполняют определённые действия в зависимости от истинности условия или выполнять код многократно.

    // ВЫРАЖЕНИЯ if
    // Позволяют разветвлять код в зависимости от условий.
    
    let number = 3;  // Если изменим переменную, на значение, которое больше 5, тогда выполним блок кода else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Условие должно быть логического типа bool, иначе будет ошибка
    // let number = 4;
    // if number {                 // ^^^^^^ expected `bool`, found integer
    //     println!("number was three");
    // }

    let number = 3;
    if number !=0 {
        println!("number was something other than zero");
    }

    // Обработка нескольких условий с помощью выражения `else if`
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // В дальнейшем лучше использовать конструкцию ветвления match

    // ИСПОЛЬЗОВАНИЕ if в let-операторах
    // Т.к. if является выражением его можно присвоить переменной.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Значения, которые могут быть результатами каждой из ветвей if, должны быть одного типа
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // //                                       ^^^^^ expected integer, found `&str`
    // println!("The value of number is: {number}");


    // ПОВТОРЕНИЕ ВЫПОЛНЕНИЯ КОДА С ПОМОЩЬЮ ЦИКЛОВ

    // break - прерывает выполнение цикла и переходит к выполнению дальнейшего кода программы.
    // continue - позволяет продолжить цикл, снова не прерывая цикла, но текущая итерация закончиться там где стоить `continue`
    // loop, while, for - выполняют блок кода более одного раза и называются такие конструкции циклами.

    // 1. Повторение выполнения кода с помощью loop
    // loop - выполняет блок кода до бесконечности или пока явно не будет остановлен оператором `break`

    loop {
        println!("again!");
        break;     // закоментируя эту строку мы получим бесконечный цикл
    }

    // Возвращение значений из циклов
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Метки циклов для устранения неоднозначности между несколькими циклами

    // При двух циклах внешним и внутренним применение break и continue применяется к внутреннему циклю.
    // Для того чтобы из внешннего цикла выйти используя break или continue, можно использовать метки вида
    // 'имя_метки: loop {}
    // break 'имя_метки однозначно устанавливает из к какому циклю принадлежит break
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Циклы с условием while
    // В цикле учитывается проверка условий после ключевого слова while.
    // Код выполняется, пока условие истинно.
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIGTOFF");


    // Цикл по элементам коллекции с помощью for
    // Перебрать элементы коллекции мы можем через while, организую счётчик индексов.

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // Код с использованием for выполняется по элементам массива, не высчитывая явно их индекс
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Пример обратного отвчета на for с использованием range - генератора и функции rev() - т.е. reverce (разворот, переворот)
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}   

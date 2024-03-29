/*
    4.      Понимание Владения (Understanding Ownership)
    4.1.    Что такое владение? (What Is Ownership?)
*/


fn main() {

    // Владения обеспечивают безопастность памяти без использования сборщика мусора.
    // С владениями связаны следующие возможности: заимствования. срезы, раскладывание данных в памяти.
    
    // Управление памятью просходит через систему владения с набором правил, котороные проверяет компилятор.

    // Правила владения:
    // У каждого значения в Rust есть владелец,
    // У значения может быть только один владелец в один момент времени,
    // Когда владелец покидает область видимости, значение удаляется.
    
    // В стеке значнгия хранятся в порядке их получения, а удаляет в обратном. FIFO.
    // Данные которые хранятся в стеке должны иметь известный фиксированный размер.
    // Данные размер которых во время компилящии неизвестен или может измениться, должны храниться в куче.
    


    // Область видимости переменной.
    // Область видимости - это диапазон внутри программы, для которого допустим элемент.
    {                           // s здесь не валидный т.к. он ещё не объявлен
        let s = "hello";        // s действительныйй с этого момента и далее
        // делаем что-то с s
    }                           // эта область теперь закончилась, и s больше недействителен

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let s = String::from("hello");      // s действителен с этого момента и далее
                                 // делаем что-то с s
    }                                   // эта область теперь закончилась, и s больше недействителен

    //СПОСОБЫ ВЗАИМОДЕЙСТВИЯ ПЕРЕМЕННЫХ И ДАННЫХ: ПЕРЕМЕЩЕНИЕ.
    // Так как значения постоянный размер, при присваиванияя let y = x,
    // создаётся копия значения x и размещается в стеке.
    let x = 5;
    let y = x;
    println!("{}", x);

    // Стринг в отличии от литерального представления, создание через String::from()
    // приводит к тому что переменная создаётся в куче и в ней храниться адрес (указатель) на 
    // область памяти в куче, длина и ёмкость.
    // Длина - количество байт памяти, которое использует содержимое String в данный момент.
    // Ёмкость - общее количество байт памяти, которое String получил от операционной системы. 
    let s1 = String::from("hello");
    let s2 = s1;

    // После копирования s1 в s2, происходит копирование указателя, длины, ёмкости, но не самого
    // объекта в куче, по-этому s1 и s2 указывают на один объект в памяти.

    // Когда s1 и s2 выходят из области видимости, обе перемынные будут пытаться освободить одну и ту же память в куче.
    // Это вызывает "ошибка двойного освобождения" (Double free), что является одной из ошибок
    // безопасности памяти и может привести к повреждению памяти.

    // По-этому после строки let s2 = s1, s1 перестаёт быть действительным и Rust не нужно ничего освобождать, кода s1 выходит за 
    // пределы области видимости.
    
    // println!("{}", s1);          // т.к. s1 более не существует, то вывод этой переменной приводит к ошибке
    //                  ^^ value borrowed here after mov

    // Данное действие сложно назвать копированием, такое действие называется `замена`, т.к. при копировании в переменную копируется значения предыдущей и предыдущая удаляется тоже.



    //СПОСОБЫ ВЗАИМОДЕЙСТВИЯ ПЕРЕМЕННЫХ И ДАННЫХ: КЛОНИРОВАНИЕ.
    // Данный код в отличии от перемещения, глубоко копирует данные кучи и в итоге у нах два обехкта в куче s1 и s2 ссылаются каждый к своему объекту.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //СТЕКОВЫЕ ДАННЫЕ: КОПИРОВАНИЕ.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Вот некоторые типы, которые реализуют типаж Copy:

    // Все целочисленные типы, такие как u32,
    // Логический тип данных bool, возможные значения которого true и false,
    // Все числа с плавающей запятой, такие как f64,
    // Символьный тип char,
    // Кортежи, но только если они содержат типы, которые также реализуют Copy. 
    // Например, (i32, i32) будет с Copy, но кортеж (i32, String) уже нет.


    // ВЛАДЕНИЕ И ФУНКЦИИ
    // 1 группа примеров
    let s = String::from("hello");      // s входит в область видимости 
    takes_ownership(s);                 // значение s передаётся внутрь функции
                                        // Здесь переменная s более не действует

    let x = 5;                          // x входит в область видимости
    makes_copy(x);                      // будет перемещен в функцию, но i32 - это Copy, 
                                        // поэтому можно продолжать использовать x после
    println!("{}", x);                  

    //ВОЗВРАЩЕНИЕ ЗНАЧЕНИЙ И ОБЛАСТЬ ВИДИМОСТИ
    // 2 группа примеров
    let s1 = gives_ownership();         // gives_ownership перемещает возвращаемое значение в s1
    let s2 = String::from("hello");     // s2 входит в область видимости 

    let s3 = takes_and_gives_back(s2);  // s2 перемещается в takes_and_gives_back, который также перемещает его возвращаемое значение в s3


    // ВОЗВРАТ ПРАВА ВЛАДЕНИЯ НА ПАРАМЕТРЫ
    // 3 группа примеров
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len)

}                           //1 группа примеров. Здесь x выходит за рамки, затем s. Но поскольку значение s было перемещено, ничего особенного не произошло.
                            //2 группа примеров. Здесь s3 выходит за пределы области видимости и отбрасывается. s2 был перемещен, так что ничего не происходит. s1 выходит за пределы области видимости и отбрасывается.

// 1 группа примеров
fn takes_ownership(some_string: String){    // some_string входит в область видимости 
    println!("{}", some_string);
}   //Здесь some_string выходит за рамки и вызывается «drop». Резервная память освобождается.

fn makes_copy(some_integer: i32){       // some_integer входит в область видимости.
    println!("{}", some_integer);
}   // Здесь some_integer выходит за рамки. Ничего особенного не происходит.


// 2 группа примеров

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// Эта функция принимает строку и возвращает её
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 3 группа примеров
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();   // функция len() вернёт размер строки
    (s, length)
}

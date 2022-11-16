/*
    6.      Перечисления и сопоставление с образцом (Enums and Pattern Matching)
    6.1.    Определение перечисления (Defining an Enum)
*/


/*
   Перечисления и сопоставление с образцом  (Enums and Pattern Matching)
   В этой главе мы рассмотрим перечисления (enumerations), также называемые enums. 
   Перечисления позволяют определять типы путём перечисления их возможных вариантов. 
   Во-первых, мы определим и используем перечисление, чтобы показать, как оно может объединить значения и данные. 
   Далее мы рассмотрим особенно полезное перечисление Option, которое указывает, что значение может быть или чем-то, или ничем. 
   Затем мы посмотрим, как сопоставление шаблонов в выражении match позволяет легко запускать разный код для разных значений перечислений. 
   Наконец, мы узнаем, насколько конструкция if let удобна и лаконична для обработки перечислений в вашем коде.
*/

/* Структуры описываются группу связанных полей и данные, перечисления описывают данные, которые являются одним из возможных наборов значений */

/* Пример. Допустим возьмём любой IP-адрес и он может быть 4 или 6 версии, но не обеими версиями одновременно. Для таких случаев существует enum */


// Определение простого перечисления
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Для того чтобы хранить данные об IP адресах можно создать структуру, в которой храниться сам адресс и его версия (enum).
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Это новое определение перечисления гласит, что оба варианта будут иметь соответствующие значения String.
#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){

        // method body would by defined here
    }
}


fn main() {

    // Значения перечислений (Enum Values)

    /*
    *   enum IpAddrKind {
    *       V4,
    *       V6,
    *   }
    */

    // Присваивание значения перечисления переменной, обращаясь к элементу enum через имя типа используя `::`.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Можем их передать в функцию
    route(&four);
    route(&IpAddrKind::V6);

    println!("{:?}",four as i8);

    /*
    *    struct IpAddr {
    *        kind: IpAddrKind,
    *        address: String,
    *    }
    */

    // Применение
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    /*
        Обратите внимание, что хотя определение перечисления IpAddr есть в стандартной библиотеке, 
        мы смогли объявлять и использовать свою собственную реализацию с аналогичным названием без 
        каких-либо конфликтов, потому что мы не добавили определение стандартной библиотеки в область 
        видимости кода.
    */


    /*
    *    enum IpAddrEnum {
    *        V4(String),
    *        V6(String),
    *    }
    */

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));


    // Можно каждый вариает перечисления иметь разное количество ассощиированных данных представленных в разных типах (что-то вроде кортежа)
    
    /*
        enum IpAddrEnum2 {
            V4(u8, u8, u8, u8),
            V6(String),
        }
    */

    let home = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));

    // Можно внутри Enum, в качестве типа элемента перечисления, использовать тип, который мы создаём через struct 

    /*
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    */


    // Следующий пример
    /*

        enum Message {
            Quit,
            Move { x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32, i32),
        }
    

        Это перечисление имеет 4 элемента:
        * Quit - пустой элемент без ассоциированных данных.
        * Move - имеет именованные поля, как и структура.
        * Write - элемент с единственной строкой типа String
        * ChangeColor - кортеж из трёх значений типа i32.


        // Так мы бы определили enum Message структурами
        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct
    */

    // К перечислению мы можем сделать методы:
    /*
        impl Message {
            fn call(&self){
                
                // method body would by defined here
            }
        }
    */

    let m = Message::Write(String::from("hello"));


    // Перечисление Option и его преимущества перед Null-значениями (The Option Enum and Its Advantages Over Null Values)
    
    /*
        Перечисление Option<T> представляет концепцию наличия значения или его отсутствие. В коде эта концепция представлена в виде:

        enum Option<T> {
            None,           - Отсутствие значения
            Some(T),        - Наличие значения
        }

        Данное перечисление входит в стандартную библиотеку (prelude) и его можно использовать без явного ввода в область видимости 
    */

    let some_number = Some(5);      // В some_number тип Option<i32>
    let some_char = Some('e');      // В some_char тип Option<char>
    let absent_number: Option<i32> = None;      // Для того чтобы в переменную absent_number разместить None нам нужно явно задать тип для Option<тип_данных>


    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    //          ^ no implementation for `i8 + Option<i8>`

    // Так как Option<T> (Option<i8>) и T (i8) относятся к разным типам. Компилятор данной операции не допустит.




}

    // В функцию в качестве аргумента можно передавать Enum
    fn route(ip_kind: &IpAddrKind) {
        println!("{:?}",ip_kind)
    }
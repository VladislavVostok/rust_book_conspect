use std::io;    // Включение в область видимости библиотеки ввода/вывода `io` из стандартной библиотеки `std`
use rand::Rng;  // Добавляем в область видимости типаж (trait) `Rng`, для определения методов реализующих    
                // генератор случайных чисел иp библиотеки/крейта (crate) rand.
use std::cmp::Ordering; // Вводим перечесление `enum` Ordering, в область видимости, которое имеет варианты `Less`, `Greater` и `Equal`. 


fn main() {     // Функция `main` точка входа в программу
    println!("Guess the number!");      // макрос печати строки на экран
    
    let secret_number = rand::thread_rng()   // `thread_rng` - инициализация ГСЧ локальный для текущего потока.
                                .gen_range(1..=100);        // `gen_range` - принимает в аргумент выражение диапазона 
                                                            //  и генерирует из этого диапазона случайное число.
                                                            // Форма принимаемого аргумента `start..=end`
    
    //println!("The secret number is: {secret_number}");      // Вывод в консоль текста со значение `secret_number`  
    
    loop {                                                  //Бесконечный цикл
        println!("Please input your guess.");
        
        let mut guess = String::new();
                                                                    // let apples = 5;         //Не изменяемая переменная, по-умолчанию все переменные не изменяемые
                                                                    // let mut bananas = 5;    //Изменяемой переменная становиться после `mut` - mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");                         // Паника
        
        // let guess: u32 = guess                                  // Затенение предыдущей переменной `guess`
        //                     .trim()                             // Т.к. переменная имеет тип String, то метод `trim()` удаляет пробелы.
        //                                                         // Также удаляет escape-последовательности из строки такие как `\t`, `\n`, '\r`
        //                     .parse()                            // Метод `parse()` переводит строку в переменную с явно заданным типом данных например `u32`
        //                                                         // let guess: u32  - явное аннотирование переменной типа данных
        //                     .expect("Please type a number!");   // Паника! Если тип `Result` бутет `Err`

        let guess: u32 = match guess.trim().parse(){            // Результат match присваивается переменной guess                      
            Ok(num) => num,                                     // Результат `Ok` возвращаем число
            Err(_) => continue,                                 // Результат `Err` тогда `continue` не выходя из цикла
        };  

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {                   // Метод `cmp` сравнивает два значения, принимает ссылки на значения и возвращает вариант перечисления `Ordering`
                                                            // `match` состоит из веток (arms)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;                                      //Выход из цикла
            },
        }
    }    
}

/*
    4.      Понимание Владения (Understanding Ownership)
    4.3.    Тип срезы (The Slice Type)
*/

/*
    Срезы позволяют ссылаться на часть непрерывной последовательность элементов в коллекции, а не на всю коллекцию.
    Срез не имеет права владения. 
*/

fn main() {

    /*
        Напишем функцию, которая принимает строку слов, разделённых пробелами, и возвращает первое слово, которое она
        находит в этой строке. Если функция не находит пробела в строке, вся строка должна состоять из одного слова,
        поэтому должна быть возвращена вся строка.
    */

    /* Сигнатура фукции без использования срезов */
    let mut s = String::from("hello world");
    let word = first_word(&s); // Переменная word получет от функции first_word значение 5
    println!("{} = {}", word, s);
    s.clear();   // Очищаем строку и теперь строка пустая ""
    println!("{} = {}", word, s);
    // слово по-прежнему имеет здесь значение 5, но больше нет строки, 
    // с которой мы могли бы осмысленно использовать значение 5. 
    // слово теперь полностью недействительно!


    /*
        fn second_word(s: &String) -> (usize, usize) {

        Теперь мы отслеживаем начальный и конечный индекс, и у нас есть ещё больше значений, 
        которые были рассчитаны на основе данных в определённом состоянии, но вообще не привязаны 
        к этому состоянию. У нас есть три несвязанные переменные, которые необходимо синхронизировать.
    
    */

    /* Для преодоления данной проблемы есть СТРОКОВЫЕ СРЕЗЫ */
    // Обозначается СРЕЗ так:
    // [starting_index..ending_index]
    // starting_index — это первая позиция.
    // ending_index конечный_индекс — это на единицу больше, чем последняя позиция в срезе.

    let s = String::from("hello world");
    let hello = &s[0..5];           // Переменная hello это ссылка на часть String
    let world = &s[6..11];          // На рисунке показано куда ссылкает и что хранит ссылка на срез строки


    //                s                               heap
    //               |----------|-------|            |----------|-------|
    //               | name     | value |            |   name   | value |
    //               |----------|-------|            |----------|-------|
    //               |  ptr     |   ---------------->|     0    |   h   |
    //               |----------|-------|            |----------|-------|
    //               |  len     |   5   |            |     1    |   e   |
    //               |----------|-------|            |----------|-------|
    //               | capacity |   5   |            |     2    |   l   |
    //               |----------|-------|            |----------|-------|
    //                                               |     3    |   l   |
    //                                               |----------|-------|
    //                word                           |     4    |   o   |
    //               |----------|-------|            |----------|-------|
    //               | name     | value |            |     5    |       |
    //               |----------|-------|            |----------|-------|
    //               |  ptr     |   ---------------->|     6    |   w   |
    //               |----------|-------|            |----------|-------|
    //               |  len     |   5   |            |     7    |   o   |
    //               |----------|-------|            |----------|-------|
    //                                               |     8    |   r   |
    //                                               |----------|-------|
    //                                               |     9    |   l   |
    //                                               |----------|-------|
    //                                               |    10    |   d   |
    //                                               |----------|-------|


    // Если начинать срез с 0 то можно упростить
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // Если срез включает последний байт, можно отбросить конечный номер
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    //Можно отбросить оба значения и мы получим часть всей строки
    let s = String::from("hello world");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    /*
        Примечание. 
        Индексы диапазона срезов строк должны располагаться на допустимых границах символов UTF-8. 
        Если вы попытаетесь создать фрагмент строки нарушая границы символа в котором больше одного 
        байта, ваша программа завершится с ошибкой. В целях введения срезов строк мы предполагаем, 
        что в этом разделе используется только ASCII;
    */

    /* Сигнатура фукции с использованием срезов */  
    let mut s = String::from("hello world");
    let word = first_word_2(&s); // Переменная word получает ссылку на срез слова
    println!("{} = {}", word, s);
    s.clear();   // Очищаем строку и теперь строка пустая ""
    //println!("{} = {}", word, s);
    // Так как у нас ссылки не изменяемые и очистив строку ссылка word становиться неактивной и при попытке использовать word у нас будет ошибка

    /*
        Напомним из правил заимствования, что если у нас есть неизменяемая ссылка на что-то, 
        мы не можем также взять изменяемую ссылку. Поскольку для clear необходимо обрезать String, 
        необходимо получить изменяемую ссылку. println! после вызова clear использует ссылку в word, 
        поэтому неизменяемая ссылка в этот момент всё ещё должна быть активной. Rust запрещает одновременное 
        существование изменяемой ссылки в формате clear и неизменяемой ссылки в word, и компиляция завершается 
        ошибкой. Rust не только упростил использование нашего API, но и устранил целый класс ошибок во время компиляции!
    */

    /* Строковые литералы - это срезы */

    /* 
        Напомним, что мы говорили о строковых литералах, хранящихся внутри бинарного файла. 
        Теперь, когда мы знаем чем являются срезы, мы правильно понимаем что такое строковые литералы 
    */

    let s = "Hello, world!";
    
    /*
        Тип s здесь &str: это срез, указывающий на эту конкретную точку двоичного файла. 
        Вот почему строковые литералы неизменяемы; &str — неизменяемая ссылка.
    */


    /* Строковые срезы как параметры */

    /*
        Знание того, что вы можете брать срезы литералов и String значений, 
        приводит нас к ещё одному улучшению first_word, и это его сигнатура:

            fn first_word(s: &String) -> &str {

        Можно улучшить сигнатуру, она более продвинутая, 
        потому что она позволяет нам использовать одну и ту же функцию 
        как для значений &String, так и для значений &str.
            
            fn first_word(s: &str) -> &str {

        Если у нас есть фрагмент строки, мы можем передать его напрямую. 
        Если у нас есть String, мы можем передать часть String или ссылку на String. 
        Эта гибкость использует преимущества приведения deref функции.

        Определение функции для получения фрагмента строки вместо ссылки на String делает 
        наш API более общим и полезным без потери какой-либо функциональности
    */

    let my_string = String::from("hello world");    // Это тип $String
    let word = first_word_3(&my_string[0..6]);
    let word = first_word_3(&my_string[..]);
    let word = first_word_3(&my_string);

    let my_string_literal = "hello world";      // Строковый литерал это тип &str
    let word = first_word_3(&my_string_literal[0..6]);
    let word = first_word_3(&my_string_literal[..]);

    let word = first_word_3(my_string_literal);


    /* Другие срезы */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    /*
        Этот срез имеет тип &[i32]. Он работает так же, как и срезы строк, сохраняя ссылку на первый элемент и его длину. 
        Вы будете использовать этот вид фрагмента для всех видов других коллекций.
    */


    /*
        Итоги 

        Концепции владения, заимствования и срезов обеспечивают безопасность памяти в программах на Rust во время компиляции. 
        Язык Rust даёт вам контроль над использованием памяти так же, как и другие языки системного программирования, но то, 
        что владелец данных автоматически очищает эти данные, когда владелец выходит за рамки, означает, что вам не нужно 
        писать и отлаживать дополнительный код, чтобы получить этот контроль.
    */
}

/* Сигнатура фукции без использования срезов */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();       // Преобразование строки в массив байтов.

    for(i, &item) in bytes.iter().enumerate() {     // iter() - итератор для прохода по массиву, он возвращает элемент коллекции. 
                                                    // enumerate() - оборачивает результат iter() и добавляет индекс элемента в коллекции возвращая кортеж (index, ссылка на элемент).
        if item == b' ' {
            return i;               // Если находим индекс байта указывающего на конец первого слова в строке возвращаем индекс `i`
        }
    }
    s.len()                         // Иначе возвращаем длинну строки.
}

/* Сигнатура фукции с использованием срезов */
fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,  &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


/* Сигнатура фукции с использованием срезов вместо ссылки на строку мы используем &str ссылку на срез */
fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,  &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
//Основным стилем для имён функций и переменных является змеиный регистр (snake case)

fn main() {

    // Вызов простой функции
    another_function();
    
    // Вызов функции с параметром
    another_function_2(6);

    // Вызов функции с несколькими параметрами
    print_labeled_measurement(5, 'h');

    // Операторы и выражения
    // Операторы - это инструкции, которые выполняют какое-либо действие и не возвращают значение.
    // Выражения - вычисляют результирующее значение
    let y = 6;   // Оператор, но 6 это выражение оператора let y;

    // Определение функции тоже оператор 
    // Нельза присвоить значение оператора let другой переменной, т.к. операторы не возвращают значение.
    // let x = (let y = 6); // данный пример вызывает ошибку
    // Выражением могут быть математические операции, т.к. они вычисляют значение, вызовы функций, вызов макроса.
    // Новый блок обасти видимости, созданный с помощью фигурных скобок является выражением.
    let z = {
        let x = 3;
        x + 1         // для того чтобы это было выражением и мы смогли его вернуть в оператор, в конце выражения x + 1 не ставиться `;`
    };
    println!("The value of z is: {z}");

    // Вызов функции с возвращаемыми значениями
    // Данная функция неявно возвращает значение, без использования ключевого слова return.
    // Все возвращаемые типы данных значений перечисляются после `->`
    // Данный вид функции является "абсолютно корректной функцией"
    let w = five();
    println!("The value of w is: {w}");

    let u = plus_one(5);
    println!("The value of u is: {u}");


}

// Простая функция
fn another_function(){              
    println!("Another function.");
}

// Функции с параметром
fn another_function_2(x:i32){
    println!("The value of x is: {x}");
}

// Функции с несколькими параметрами
fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// Функции с возвращаемыми значениями
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1               // Если поставим `;`, то мы получим ошику, которая говорит что функция ничего не возвращает, но мы определии после `-> i32` что вернём из функции целочисленное значение.
}
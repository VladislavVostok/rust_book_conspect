/*
    5.      Использование структур для структурирования связанных данных (Using Structs to Structure Related Data)
    5.2.    Пример использования структур (An Example Program Using Structs)
*/

/*
    Сделаем задачку. На вход в программу будут подаваться параметры длины и ширины прямоугольника в px
    и затем рассчитывать площадь прямоугольника.
*/

/* ШАГ 3 */
#[derive(Debug)]            
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    /* ШАГ 1 */
    // Описываем программу без структур
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.", 
    //     area(width1, height1)
    // );

    /* ШАГ 2 */
    /* Рефакторинг при помощи кортежей */
    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {} square pixels.", 
    //     area(rect1)
    // )

    /* ШАГ 3 */
    /* Рефакторинг при помощи структур: добавим больше смысла */


    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        area(&rect1)
    );

    /* Добавление полезной функциональности при помощи выводимых типажей */




    // Давайте попробуем! Вызов макроса println! теперь будет выглядеть так println!("rect1 is {:?}", rect1);. 
    // Ввод спецификатора :? внутри фигурных скобок говорит макросу println!, что мы хотим использовать другой формат вывода, известный как Debug. 
    // Типаж Debug позволяет печатать структуру способом, удобным для разработчиков, чтобы видеть значение во время отладки кода.
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // Другой способ распечатать значение в формате Debug — использовать макрос dbg!, 
    // который становится владельцем выражения (в отличие от println!, принимающего ссылку), 
    // печатает номер файла и строки, где происходит вызов макроса dbg!, 
    // вместе с результирующим значением этого выражения и возвращает владение на значение.
    dbg!(&rect1);

}

/* ШАГ 1 */
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/* ШАГ 2 */
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

/* ШАГ 3 */
fn area(rectangle: &Rectangle) ->u32 {
    rectangle.width * rectangle.height
}
/*
    6.      Перечисления и сопоставление с образцом (Enums and Pattern Matching)
    6.3.    Компактное управление потоком выполнения с if let (Concise Control Flow with if let)
*/

/*
    Синтаксис if let позволяет скомбинировать if и let в менее многословную конструкцию, и затем обработать 
    значения соответствующе только одному шаблону, одновременно игнорируя все остальные.  
*/


fn main() {

    let config_max = Some(3u8);
    match config_max {
        Some(max) = println!("The maximum is configured to be {}", max),
        _ => (),
    }

    /* Коротко это можнос сделать с помощью if let, т.к. в match нужно добавлять условие для всех остальных вариантов. */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /* 
        Синтаксис if let принимает шаблон и выражение, разделённые знаком равенства. if let сработает так же, как match, 
        когда в него на вход передадут выражение и подходящим шаблоном для этого выражения окажется первая ветка. 
        В нашем случае шаблоном является Some(max), где max привязывается к значению внутри Some. Затем мы можем использовать 
        max в теле блока if let так же, как мы использовали max в соответствующей ветке match. Код в блоке if let не запускается, 
        если значение не соответствует шаблону.

        Тут мы теряем полноту проверок
    */

    // Можно добавлять else к if let

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabaam,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
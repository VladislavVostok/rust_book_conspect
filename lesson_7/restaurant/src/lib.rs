/* Создан библиотечный crate restaurant */

// В модули можно размещать другие модули, а также могут содержать определения для структур, перечислений, констант, типажей, функций.
// Таким образом создаётся связанность кода к определённой абстракции.

mod front_of_house {                // Определение модуля с именем front_of_house Родитель(Parent) для двух модулей
    mod hosting {                   // Определение подмодуля hosting в модуле front_of_house Потомок (child) родительского модуля
        fn add_to_waitlist() {}     
        fn seat_at_table() {}
    }

    mod serving {                   // Потомок (child) родительского модуля
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    /* Модули hosting и serving являются братьями (siblings) так как они определены в одном модуле */
}

// Структура модуля с подмодулями
//  crate
// |______ front_of_house
//        |_______ hosting
//        |       |___ add_to_waitlist
//        |       |___ seat_at_table
//        |_______ serving
//                |___ take_order
//                |___ serve_order
//                |___ take_payment 

// Пути для ссылки на элемент в дереве модулей
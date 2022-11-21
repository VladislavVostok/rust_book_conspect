mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

use crate::front_of_house::hosting;     // Подключение к области видимости модуля hosting и теперь по короткому пути нам доступны функции из этого модуля

// pub fn eat_at_restaurant(){
//     hosting::add_to_waitlist();
// }

/* Если поместим функцию eat_at_restaurant в модуль customer и он будет не виден
    тут два пути решения проблемы:
    1) super
    2) это использовать use crate::front_of_house::hosting внетри модуля customer
*/

mod customer {
    use crate::front_of_house::hosting;         // 2 вариант
    pub fn eat_at_restaurant(){
        // super::hosting::add_to_waitlist();   // 1 вариант
        hosting::add_to_waitlist();
    }
}
/* Пути для ссылки на элемент в дереве модулей */

// Виды путей:
// Абсолютный путь - это полный путь, начинающийся от корневого модуля крейта;
// для кода из внешнего крейта абсолютный путь начинается с имени крейта, а для кода из текущего крейта он начинается с литерала crate.

// Относительный путь начинается с текущего модуля и использует ключевые слова self, super или идентификатор в текущем модуле.

// Абсолютные и относительные пути состоят из одного или нескольких идентификаторов, разделённых двойными двоеточиями (::)

mod front_of_house {                // по-умолчанию модуль является приватным и из родительского модуля он будет недоступен, 
                                    // но в дочерних модулях можно использовать функционал родительсткого модуля
                                    // Для того чтобы сделать его доступным нужно использовать ключевое слово pub перед mod
    pub mod hosting {               //
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();  // Абсолютный путь

    front_of_house::hosting::add_to_waitlist();         // Относительный путь
}

/* Начинаем относительный путь с помощью super */

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast{
        pub toast: String,              // Доступное поле структуры
        seasonal_fruit: String,         // Приватное поле структуры
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();     // Получаем функцию из родительского модуля. Относительный путь через super
    }

    fn cook_order(){}

    pub fn eat_at_restaurant_2() {
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        meal.seasonal_fruit = String::from("blueberries");  // Вроде должна быть ошибка
        println!("I'd like {} toast please", meal.seasonal_fruit);
        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
    }
}
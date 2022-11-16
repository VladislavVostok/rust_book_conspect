/*
    5.      Использование структур для структурирования связанных данных (Using Structs to Structure Related Data)
    5.1.    Определение и инициализация структур (Defining and Instantiating Structs)
*/


/*
    Структура (struct) = это пользовательский тип данных, позволяющий назвать и упаковать вместе несколько связанных значений,
    составляющих значимую логическую группу.

    Структуры и перечисления являются строительными блоками для создания новых типов в предметной области вашей программы.

    Объявление структуры - это как шаблон нашего типа, в то время как экземпляр структуры использует этот шаблон, 
    заполняя его определёнными данными, для создания значений нашего типа
*/

// обявляем структуру как тип User
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Объявляем кортежные структуры

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Объявляем единично-подобную структуру
struct AlwaysEqual;

// Объявление структуры с сылками на тип строковый срез
struct UserPtr {
    active: bool,
    //username: &str,         // Требует указать время жизни ссылки, пока закоментируем, в главе 10 будет понятней
    //email: &str,
    sign_in_count: u64,
}


fn main() {

    /* Определение и инициализация структур */

    // Инициализируем экземпляр структуры типа User заполняя его значениями.
    let mut user1 = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);    // Используя имя экземпляра структуры через модификатор доступа `.` мы можем получить доступ к значению его полей.

    user1.email = String::from("anotherremail@example.com");
    println!("{}", user1.email);    // По модификатору доступа `.` можно менять значения полей экземпляра структуры, но важно чтобы при инициализации 
                                    // использовался `mut` перез именем переменной.

    // можем неявно возвращать новый экземпляр из тела функции. Создадим функцию build_user и передадим в качестве 
    // аргументов функции email и username и вернём экземпляр.
    let email = String::from("someemail@mail.ru");
    let username = String::from("someusername666");
    let user2 = build_user(email, username);

    println!("{}, {}", user2.email, user2.username);


    /* Использование сокращённой инициализации поля */
    let email = String::from("shalfey@mail.ru");
    let username = String::from("shalfey666");
    let user3 = build_user_short_init_field(email, username);
    println!("{}, {}", user3.email, user3.username);


    /* Создание экземпляра структуры из экземпляра другой структуры с помощью синтаксиса обновления структуры */

    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user4.username);

    // Синтаксис `..` указывает, что оставшиеся поля устанавливаются неявно и должны иметь значения из указанного экземпляра

    let user5 = User {
        email: String::from("myemail@bk.ru"),
        ..user3
    };

    // Стоит отметить, что синтаксис обновления структуры использует = как присваивание. Это связано с перемещением данных, 
    // как мы видели в разделе "Способы взаимодействия переменных и данных: перемещение". В этом примере мы больше не можем 
    // использовать user3 после создания user5, потому что String в поле username из user3 было перемещено в user5. Если бы мы 
    // задали user5 новые значения String для email и username, и при этом использовать только значения active и sign_in_count из 
    // user3, то user1 все ещё будет действительным после создания user5. Типы active и sign_in_count являются типами, реализующими 
    // типаж Copy, поэтому будет применяться поведение, о котором мы говорили в разделе "Стековые данные: Копирование". 


    /* Кортежные структуры: структуры без именованных полей для создания разных типов */

    // Не понял для чего используется.
    // Каждая опреляемая структура имеет собственный тип.
    // В остальном всё как обычный кортеж

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} = {}", black.1, origin.1);


    /* Единично-подобные структуры: структуры без полей */

    // Они называются единично-подобными структурами, поскольку ведут себя аналогично () - unit это кортеж без каких-либо значений
    // Полезны когда нужно создать Типаж
    let subject = AlwaysEqual;



    /* Владение данными структуры */

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/*
    Здесь происходит создание нового экземпляра структуры User, которая имеет поле с именем email. 
    Мы хотим установить поле структуры email значением входного параметра email функции build_user. 
    Так как поле email и входной параметр функции email имеют одинаковое название, можно писать просто 
    email вместо кода email: email
*/

fn build_user_short_init_field(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
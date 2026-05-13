pub fn run() {
    // ===== Capitolul 2: Memory, variables, and ownership =====
    lunches_2_1();
    lunches_2_2();
}

fn lunches_2_1() {
    // ----- 2.1 The stack, the heap, pointers, and references -----
    // Exemple de references, String și &str, plus dimensiuni în bytes.

    let my_number = 15;
    let single_reference = &my_number;
    let double_reference = &single_reference;
    let five_references = &&&&&my_number;

    println!("my_number = {}", my_number);
    println!("single_reference = {}", single_reference);
    println!("double_reference = {}", double_reference);
    println!("five_references = {}", five_references);

    let my_str = "Hello, world!";
    let my_string = String::from("Hello, world!");

    println!(
        "A String is Sized and always {} bytes.",
        std::mem::size_of::<String>()
    );
    println!(
        "A &str like '{}' is {} bytes.",
        my_str,
        std::mem::size_of_val(my_str)
    );
    println!("Owned String value: {}", my_string);

    let other_string = "This is the string text".to_string();
    let formatted_string = format!("{} plus format!", other_string);
    println!("{}", formatted_string);
}

fn lunches_2_2() {
    // ----- 2.2 Strings -----
    // Diferențe între `String` și `&str`, plus exemple UTF-8 și mărimi în bytes.

    let name = "자우림";
    let other_name = String::from("Adrian Fahrenheit Țepeș");
    println!("&str UTF-8 example: {}", name);
    println!("String UTF-8 example: {}", other_name);

    let emoji_name = "🦀";
    println!("My name is actually {}", emoji_name);

    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("A String is Sized and always {size_of_string} bytes.");
    println!("An i8 is Sized and always {size_of_i8} bytes.");
    println!("An f64 is always Sized and {size_of_f64} bytes.");
    println!("But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.");
    println!(
        "And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not Sized."
    );

    let from_string = String::from("This is the string text");
    let to_string_string = "This is the string text".to_string();
    let formatted = format!("{} + {}", from_string, to_string_string);
    println!("{}", formatted);

    let into_string: String = "Try to make this a String".into();
    println!("into() with type annotation: {}", into_string);

    // Exemplul invalid din carte (doar ca notă, nu compilează):
    // let my_name: str = "My name";
    // Corect: let my_name: &str = "My name";
}

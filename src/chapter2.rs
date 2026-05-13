pub fn run() {
    // ===== Capitolul 2: Memory, variables, and ownership =====
    lunches_2_1();
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

pub fn run() {
    // ===== Capitolul 2: Memory, variables, and ownership =====
    lunches_2_1();
    lunches_2_2();
    lunches_2_3();
    lunches_2_4();
    lunches_2_5();
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

const NUMBER_OF_MONTHS: u32 = 12;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

fn print_months() {
    println!("Number of months in the year: {NUMBER_OF_MONTHS}");
}

fn lunches_2_3() {
    // ----- 2.3 const and static -----
    // Exemple de valori globale declarate cu `const` și `static`.

    print_months();
    println!("Seasons count: {}", SEASONS.len());
    println!("First season: {}", SEASONS[0]);

    // Rust nu face type inference pentru const/static, de aceea tipul e explicit.
    const DAYS_IN_WEEK: u8 = 7;
    println!("Days in a week: {}", DAYS_IN_WEEK);

    // Ideea principală: const/static trăiesc pe toată durata programului.
}

fn lunches_2_4() {
    // ----- 2.4 More on references -----
    // Referințe multiple immutable și de ce nu poți returna referință la date locale.

    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;

    println!("country via ref_one: {}", ref_one);
    println!("country via ref_two: {}", ref_two);

    // Exemplu invalid din carte (nu compilează, explicativ):
    // fn return_str() -> &String {
    //     let country = String::from("Austria");
    //     let country_ref = &country;
    //     country_ref
    // }
    // Problema: `country` moare la finalul funcției, iar referința ar deveni dangling.

    let country_owned = return_owned_country();
    println!("owned return works: {}", country_owned);
}

fn return_owned_country() -> String {
    String::from("Austria")
}

fn lunches_2_5() {
    // ----- 2.5 Mutable references -----
    // `&mut` permite schimbarea datelor împrumutate prin dereferencing (`*`).

    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("mutable ref changed number to: {}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!(
        "Are they equal? {}",
        second_number == ***triple_reference
    );

    // 2.5.1 / 2.5.2 / 2.5.3 / 2.5.4 – reguli și situații.
    situation_1_only_one_mutable_reference();
    situation_2_only_immutable_references();
    situation_3_problem_situation_explained();
    non_lexical_lifetime_valid_pattern();
}

fn situation_1_only_one_mutable_reference() {
    // Situația 1: un singur mutable reference este OK.
    let mut value = 10;
    let value_change = &mut value;
    *value_change += 5;
    println!("Situation 1 (one mutable ref): {}", value);
}

fn situation_2_only_immutable_references() {
    // Situația 2: oricâte immutable references sunt OK.
    let value = String::from("Presentation");
    let r1 = &value;
    let r2 = &value;
    let r3 = &value;
    println!("Situation 2 (immutable refs): {}, {}, {}", r1, r2, r3);
}

fn situation_3_problem_situation_explained() {
    // Situația 3 (problemă): immutable + mutable active în același timp => compiler error.
    // Exemplu INVALID din carte (comentat intenționat, nu compilează):
    // let mut number = 10;
    // let number_ref = &number;
    // let number_change = &mut number;
    // *number_change += 10;
    // println!("{}", number_ref);

    println!(
        "Situation 3: mixed immutable+mutable borrow at same time is rejected by compiler."
    );
}

fn non_lexical_lifetime_valid_pattern() {
    // Pattern valid din carte (NLL): mutable borrow se termină când nu mai e folosit.
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;

    let number_ref = &number;
    println!("NLL valid pattern result: {}", number_ref);
}

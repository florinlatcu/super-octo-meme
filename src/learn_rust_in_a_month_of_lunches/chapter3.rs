pub fn run() {
    // ===== Capitolul 3: More complex types =====
    lunches_3_1();
    lunches_3_1_1();
    lunches_3_1_2();
    lunches_3_1_3();
    lunches_3_2();
    lunches_3_2_1();
    lunches_3_2_2();
}

fn lunches_3_1() {
    println!("\n=== 3.1 Collection types ===");
    // ----- 3.1 Collection types -----
    // În 3.1 introducem trei colecții: arrays, vectors și tuples.

    let array_example = ["Bucharest", "Cluj"];
    let vector_example = vec!["Seoul", "Busan", "Tokyo"];
    let tuple_example = ("name", 8, true);

    println!("[3.1] Array: {:?}", array_example);
    println!("[3.1] Vec: {:?}", vector_example);
    println!("[3.1] Tuple: {:?}", tuple_example);

    print_type_of("array_example", &array_example);
    print_type_of("vector_example", &vector_example);
    print_type_of("tuple_example", &tuple_example);
}

fn lunches_3_1_1() {
    println!("\n=== 3.1.1 Arrays ===");
    // ----- 3.1.1 Arrays -----

    // Arrays de lungimi diferite => tipuri diferite.
    let array1 = ["One", "Two"];
    let array2 = ["One", "Two", "Five"];
    print_type_of("array1", &array1);
    print_type_of("array2", &array2);

    // Întrebare pentru compilator (exemplu INVALID din carte, lăsat comentat):
    // let seasons = ["Spring", "Summer", "Autumn", "Winter"];
    // seasons.ddd();

    // Repetare de valoare cu [valoare; număr].
    let my_array = ["a"; 5];
    println!("[3.1.1] Repeated array: {:?}", my_array);

    // Buffer de bytes (foarte folosit în practică).
    let mut buffer = [0u8; 640];
    buffer[0] = 7;
    buffer[639] = 255;
    println!("[3.1.1] Buffer preview: first={}, last={}", buffer[0], buffer[639]);

    // Prefixul b => array de bytes.
    let hello_bytes = b"Hello there";
    println!("[3.1.1] b\"Hello there\" = {:?}", hello_bytes);
    print_type_of("hello_bytes", &hello_bytes);

    // Indexing.
    let my_numbers = [0, 10, -20];
    println!("[3.1.1] my_numbers[1] = {}", my_numbers[1]);

    // Slicing (exclusive/inclusive).
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    let inclusive_zero_to_two = &array_of_ten[0..=2];

    println!(
        "[3.1.1] two_to_five={:?}, start_at_one={:?}, end_at_five={:?}, everything={:?}, inclusive_zero_to_two={:?}",
        two_to_five, start_at_one, end_at_five, everything, inclusive_zero_to_two
    );
}

fn lunches_3_1_2() {
    println!("\n=== 3.1.2 Vectors ===");
    // ----- 3.1.2 Vectors -----

    // Vec::new() + push()
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    println!("[3.1.2] Vec::new + push: {:?}", my_vec);
    print_type_of("my_vec", &my_vec);

    // Tip explicit pentru Vec.
    let mut explicit_vec: Vec<String> = Vec::new();
    explicit_vec.push("Seoul".to_string());
    explicit_vec.push("Busan".to_string());
    println!("[3.1.2] Vec<String> explicit: {:?}", explicit_vec);

    // vec! macro.
    let mut vec_macro = vec![8, 10, 10];
    vec_macro.push(12);
    println!("[3.1.2] vec! macro: {:?}", vec_macro);

    // Slicing pe vector.
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];
    println!(
        "[3.1.2] slices -> three_to_five={:?}, start_at_two={:?}, end_at_five={:?}, everything={:?}",
        three_to_five, start_at_two, end_at_five, everything
    );

    // Capacitate și realocare.
    let mut num_vec = Vec::new();
    println!("[3.1.2] capacity start: {}", num_vec.capacity());
    num_vec.push('a');
    println!("[3.1.2] after 1 push: {}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("[3.1.2] after 4 pushes: {}", num_vec.capacity());
    num_vec.push('a');
    println!("[3.1.2] after 5 pushes: {}", num_vec.capacity());

    // Vec::with_capacity() pentru eficiență.
    let mut num_vec_better = Vec::with_capacity(8);
    num_vec_better.push('a');
    println!("[3.1.2] with_capacity after 1 push: {}", num_vec_better.capacity());
    num_vec_better.push('a');
    num_vec_better.push('a');
    num_vec_better.push('a');
    num_vec_better.push('a');
    println!("[3.1.2] with_capacity after 5 pushes: {}", num_vec_better.capacity());

    // Array -> Vec cu into().
    let my_vec_u8: Vec<u8> = [1, 2, 3].into();
    let my_vec_inferred: Vec<_> = [9, 0, 10].into();
    println!("[3.1.2] into vec explicit: {:?}", my_vec_u8);
    println!("[3.1.2] into vec inferred: {:?}", my_vec_inferred);
}

fn lunches_3_1_3() {
    println!("\n=== 3.1.3 Tuples ===");
    // ----- 3.1.3 Tuples -----

    // Tuple gol = unit type ().
    let unit_value = do_something_unit();
    println!("[3.1.3] unit value: {:?}", unit_value);
    print_type_of("unit_value", &unit_value);

    // Tuple cu tipuri diferite.
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!("[3.1.3] random_tuple.0 = {:?}", random_tuple.0);
    println!("[3.1.3] random_tuple.1 = {:?}", random_tuple.1);
    println!("[3.1.3] random_tuple.2 = {:?}", random_tuple.2);
    println!("[3.1.3] random_tuple.3 = {:?}", random_tuple.3);
    println!("[3.1.3] random_tuple.4 = {:?}", random_tuple.4);
    println!("[3.1.3] random_tuple.5 = {:?}", random_tuple.5);
    print_type_of("random_tuple", &random_tuple);

    // Destructurare completă.
    let strings = (
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
    );
    let (a, b, c) = strings;
    println!("[3.1.3] destructuring complet: a={a}, b={b}, c={c}");

    // Potrivire de pattern cu _ pentru elemente nefolosite.
    let tuple_of_three = ("one", "two", "three");
    let (_, second, third) = tuple_of_three;
    println!("[3.1.3] partial destructuring: second={second}, third={third}");

    // Exemplu invalid din carte (comentat):
    // let (x, y) = tuple_of_three;
    // Nu compilează: pattern-ul nu se potrivește (2 vs 3 elemente).
}

fn lunches_3_2() {
    println!("\n=== 3.2 Control flow ===");
    // ----- 3.2 Control flow -----
    // Control flow înseamnă să rulezi cod diferit în funcție de condiții.

    let test_number = 9;
    println!("[3.2] test_number = {test_number}");
    println!("[3.2] par? {}", test_number % 2 == 0);
    println!("[3.2] pozitiv? {}", test_number > 0);
}

fn lunches_3_2_1() {
    println!("\n=== 3.2.1 Basic control flow ===");
    // ----- 3.2.1 Basic control flow -----

    let my_number = 5;

    // if simplu
    if my_number == 7 {
        println!("[3.2.1] It's seven");
    }

    // if + else if + else
    if my_number == 7 {
        println!("[3.2.1] It's seven");
    } else if my_number == 6 {
        println!("[3.2.1] It's six");
    } else {
        println!("[3.2.1] It's a different number");
    }

    // condiții compuse cu && și ||
    if my_number % 2 == 1 && my_number > 0 {
        println!("[3.2.1] It's a positive odd number");
    } else if my_number == 6 || my_number == -6 {
        println!("[3.2.1] It's six in absolute value");
    } else {
        println!("[3.2.1] It's a different number");
    }

    // Notă din carte: în Rust nu ai nevoie de paranteze la if.
    // if (my_number == 7) { ... } // compilează, dar parantezele sunt inutile.
}

fn lunches_3_2_2() {
    println!("\n=== 3.2.2 Match statements ===");
    // ----- 3.2.2 Match statements -----

    // Match exhaustiv + wildcard.
    let my_number: u8 = 5;
    match my_number {
        0 => println!("[3.2.2] it's zero"),
        1 => println!("[3.2.2] it's one"),
        2 => println!("[3.2.2] it's two"),
        _ => println!("[3.2.2] it's some other number"),
    }

    // Match folosit pentru a produce o valoare.
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };
    println!("[3.2.2] second_number = {second_number}");

    // Match pe tuple.
    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("[3.2.2] It's dark and unpleasant today"),
        ("clear", "warm") => println!("[3.2.2] It's a nice day"),
        ("cloudy", "warm") => println!("[3.2.2] It's dark but not bad"),
        _ => println!("[3.2.2] Not sure what the weather is."),
    }

    // Match guard.
    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if !married => {
            println!("[3.2.2] Not married with {children} kids")
        }
        (children, married) if children == 0 && married => {
            println!("[3.2.2] Married but no children")
        }
        _ => println!("[3.2.2] Married? {married}. Number of children: {children}."),
    }

    // _ de mai multe ori într-un pattern.
    match_colors((200, 0, 0));
    match_colors((50, 50, 50));
    match_colors((200, 50, 0));

    // @ pentru a denumi valoarea potrivită în branch-ul de match.
    match_number(50);
    match_number(13);
    match_number(16);
    match_number(4);

    // Exemplu invalid din carte (comentat):
    // let some_variable = match my_number {
    //     10 => 8,
    //     _ => "Not ten",
    // };
    // Nu compilează: brațele din match trebuie să întoarcă același tip.
}

fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("[3.2.2] Not much red"),
        (_, g, _) if g < 10 => println!("[3.2.2] Not much green"),
        (_, _, b) if b < 10 => println!("[3.2.2] Not much blue"),
        _ => println!("[3.2.2] Each color has at least 10"),
    }
}

fn match_number(input: i32) {
    match input {
        number @ 4 => println!("[3.2.2] {number} is unlucky in China (sounds close to 死)!"),
        number @ 13 => println!("[3.2.2] {number} is lucky in Italy! In bocca al lupo!"),
        number @ 14..=19 => println!("[3.2.2] Some other number that ends with -teen: {number}"),
        _ => println!("[3.2.2] Some other number, I guess"),
    }
}

fn do_something_unit() {
    let _unused_number = 10;
}

fn print_type_of<T>(label: &str, _: &T) {
    println!("[type] {} => {}", label, std::any::type_name::<T>());
}
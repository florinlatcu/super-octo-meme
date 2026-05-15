pub fn run() {
    // ===== Capitolul 3: More complex types =====
    lunches_3_1();
    lunches_3_1_1();
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

fn print_type_of<T>(label: &str, _: &T) {
    println!("[type] {} => {}", label, std::any::type_name::<T>());
}
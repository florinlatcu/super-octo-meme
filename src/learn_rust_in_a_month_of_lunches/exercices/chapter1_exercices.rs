// Exerciții pentru Capitolul 1
// Notă: fiecare funcție este un exercițiu propus.
// Structură recomandată:
// - Exercițiile 1-6: Bază
// - Exercițiile 7-9: Combinate
// - Exercițiul 10: Mini-provocare

use core::ascii;

pub fn run() {
    println!("\n=== Exerciții: Capitolul 1 ===");

    println!("\n--- Nivel Bază (1-6) ---");
    exercitiu_1_1_salut_personalizat();
    exercitiu_1_2_comentarii_si_tipuri();
    exercitiu_1_3_bytes_vs_caractere();
    exercitiu_1_4_inferenta_si_suffix();
    exercitiu_1_5_operatii_cu_float();
    // exercitiu_1_6_functie_cu_return_implicit();

    println!("\n--- Nivel Combinat (7-9) ---");
    // exercitiu_1_7_bloc_care_returneaza_valoare();
    // exercitiu_1_8_debug_struct();
    // exercitiu_1_9_min_max();

    println!("\n--- Mini-provocare (10) ---");
    // exercitiu_1_10_mutabilitate_si_shadowing();
}

fn exercitiu_1_1_salut_personalizat() {
    println!("Exercițiul 1");
    // Nivel: Bază
    // Exercițiu 1:
    // Creează un String mutabil cu numele tău,
    // adaugă un caracter '!' și afișează rezultatul.
    let mut numele_meu = String::from("Florin Lațcu");
    numele_meu.push('!');
    println!("Salut, {}", numele_meu);
}

fn exercitiu_1_2_comentarii_si_tipuri() {
    println!("Exercițiul 2");
    // Nivel: Bază
    // Exercițiu 2:
    // Declară o variabilă numerică folosind inferența tipului,
    // apoi declară o a doua variabilă cu tip explicit care preia aceeași valoare.
    // Adaugă comentarii care explică ce face fiecare linie.
    let număr_inferat = 42; // Rust va deduce că este un i32
    let număr_explicit: i32 = număr_inferat; // Acum avem o variabilă cu tip explicit
    println!("Număr inferat: {}, Număr explicit: {}", număr_inferat, număr_explicit);
}

fn exercitiu_1_3_bytes_vs_caractere() {
    println!("Exercițiul 3");
    // Nivel: Bază
    // Exercițiu 3:
    // Alege două șiruri: unul ASCII și unul cu diacritice sau caractere Unicode.
    // Afișează pentru fiecare:
    // - numărul de bytes (len)
    // - numărul de caractere (chars().count())
    let ascii = "Florin!";
    let unicode = "Florin Lațcu!";
    println!("ASCII: {} bytes, {} caractere", ascii.len(), ascii.chars().count());
    println!("Unicode: {} bytes, {} caractere", unicode.len(), unicode.chars().count());
}

fn exercitiu_1_4_inferenta_si_suffix() {
    println!("Exercițiul 4");
    // Nivel: Bază
    // Exercițiu 4:
    // Declară trei numere:
    // - unul cu tip inferat
    // - unul cu tip explicit u8
    // - unul cu suffix i32 și underscore pentru lizibilitate.
    // Afișează valorile într-un singur println!.
    let număr_inferat = 100; // Rust va deduce că este un i32
    let număr_u8: u8 = 255; // Tip explicit u8
    let număr_i32 = 1_000_000i32; // Suffix i32 și underscore pentru lizibilitate
    println!("Număr inferat: {}, Număr u8: {}, Număr i32: {}", număr_inferat, număr_u8, număr_i32);
}

fn exercitiu_1_5_operatii_cu_float() {
    println!("Exercițiul 5");
    // Nivel: Bază
    // Exercițiu 5:
    // Declară un f64 și un f32, convertește când este necesar,
    // apoi calculează suma și media lor într-o variabilă separată.
    // Afișează rezultatele cu 2 zecimale.
    let număr_f64: f64 = 3.14159;
    let număr_f32: f32 = 2.71828;
    let sumă = număr_f64 + număr_f32 as f64; // Convertim f32 la f64 pentru adunare
    let medie = sumă / 2.0;
    println!("Sumă: {:.2}, Medie: {:.2}", sumă, medie);
}

fn exercitiu_1_6_functie_cu_return_implicit() {
    println!("Exercițiul 6");
    // Nivel: Bază
    // Exercițiu 6:
    // Scrie o funcție care primește doi parametri i32 și întoarce produsul lor,
    // fără a folosi cuvântul `return`.
    // Apelează funcția și afișează rezultatul.
}

fn exercitiu_1_7_bloc_care_returneaza_valoare() {
    println!("Exercițiul 7");
    // Nivel: Combinat
    // Exercițiu 7:
    // Creează o variabilă inițializată printr-un bloc `{}`.
    // În bloc, declară două numere și întoarce suma lor ca expresie finală.
    // Apoi creează încă un bloc unde pui `;` pe ultima linie și observă diferența.
}

fn exercitiu_1_8_debug_struct() {
    println!("Exercițiul 8");
    // Nivel: Combinat
    // Exercițiu 8:
    // Definește o structură simplă (de exemplu Student) cu două câmpuri.
    // Derivează `Debug`, creează o instanță și afișeaz-o cu `{:?}` și `{:#?}`.
}

fn exercitiu_1_9_min_max() {
    println!("Exercițiul 9");
    // Nivel: Combinat
    // Exercițiu 9:
    // Afișează valorile MIN și MAX pentru tipurile i8, u8, i16 și u16.
    // Bonus: adaugă și i32.
}

fn exercitiu_1_10_mutabilitate_si_shadowing() {
    println!("Exercițiul 10");
    // Nivel: Mini-provocare
    // Exercițiu 10:
    // Declară o variabilă mutabilă, schimbă-i valoarea,
    // apoi folosește shadowing pe o variabilă separată pentru a o transforma din i32 în f64.
    // Afișează fiecare etapă.
}

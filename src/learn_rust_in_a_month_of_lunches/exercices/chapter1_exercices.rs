// Exerciții pentru Capitolul 1
// Notă: fiecare funcție este un exercițiu propus.

pub fn run() {
    println!("\n=== Exerciții: Capitolul 1 ===");
    exercitiu_1_1_salut_personalizat();
    exercitiu_1_2_comentarii_si_tipuri();
    exercitiu_1_3_bytes_vs_caractere();
    exercitiu_1_4_inferenta_si_suffix();
    exercitiu_1_5_operatii_cu_float();
    exercitiu_1_6_functie_cu_return_implicit();
    exercitiu_1_7_bloc_care_returneaza_valoare();
    exercitiu_1_8_debug_struct();
    exercitiu_1_9_min_max();
    exercitiu_1_10_mutabilitate_si_shadowing();
}

fn exercitiu_1_1_salut_personalizat() {
    // Exercițiu 1:
    // Creează un String mutabil cu numele tău,
    // adaugă un caracter '!' și afișează rezultatul.
}

fn exercitiu_1_2_comentarii_si_tipuri() {
    // Exercițiu 2:
    // Declară o variabilă numerică folosind inferența tipului,
    // apoi declară o a doua variabilă cu tip explicit care preia aceeași valoare.
    // Adaugă comentarii care explică ce face fiecare linie.
}

fn exercitiu_1_3_bytes_vs_caractere() {
    // Exercițiu 3:
    // Alege două șiruri: unul ASCII și unul cu diacritice sau caractere Unicode.
    // Afișează pentru fiecare:
    // - numărul de bytes (len)
    // - numărul de caractere (chars().count())
}

fn exercitiu_1_4_inferenta_si_suffix() {
    // Exercițiu 4:
    // Declară trei numere:
    // - unul cu tip inferat
    // - unul cu tip explicit u8
    // - unul cu suffix i32 și underscore pentru lizibilitate.
    // Afișează valorile într-un singur println!.
}

fn exercitiu_1_5_operatii_cu_float() {
    // Exercițiu 5:
    // Declară un f64 și un f32, convertește când este necesar,
    // apoi calculează suma și media lor într-o variabilă separată.
    // Afișează rezultatele cu 2 zecimale.
}

fn exercitiu_1_6_functie_cu_return_implicit() {
    // Exercițiu 6:
    // Scrie o funcție care primește doi parametri i32 și întoarce produsul lor,
    // fără a folosi cuvântul `return`.
    // Apelează funcția și afișează rezultatul.
}

fn exercitiu_1_7_bloc_care_returneaza_valoare() {
    // Exercițiu 7:
    // Creează o variabilă inițializată printr-un bloc `{}`.
    // În bloc, declară două numere și întoarce suma lor ca expresie finală.
    // Apoi creează încă un bloc unde pui `;` pe ultima linie și observă diferența.
}

fn exercitiu_1_8_debug_struct() {
    // Exercițiu 8:
    // Definește o structură simplă (de exemplu Student) cu două câmpuri.
    // Derivează `Debug`, creează o instanță și afișeaz-o cu `{:?}` și `{:#?}`.
}

fn exercitiu_1_9_min_max() {
    // Exercițiu 9:
    // Afișează valorile MIN și MAX pentru tipurile i8, u8, i16 și u16.
    // Bonus: adaugă și i32.
}

fn exercitiu_1_10_mutabilitate_si_shadowing() {
    // Exercițiu 10:
    // Declară o variabilă mutabilă, schimbă-i valoarea,
    // apoi folosește shadowing pe o variabilă separată pentru a o transforma din i32 în f64.
    // Afișează fiecare etapă.
}

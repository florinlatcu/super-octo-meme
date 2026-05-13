# Notițe carte Rust 
# Learn Rust in a Month of Lunches

## 1.1

1. Secțiunea introduce primul contact practic cu Rust.
2. Ideea principală este fluxul simplu: scriu cod, compilez și rulez.
3. Programul începe din `fn main()`, care este punctul de intrare.
4. Exemplul arată folosirea unui `String` mutabil și afișarea lui în consolă.
5. Concluzia este că 1.1 te familiarizează cu baza și cu mediul de lucru.

Concepte-cheie:
- `fn main()` este punctul de intrare.
- `String` se poate modifica dacă variabila este `mut`.
- Compilatorul oferă sugestii utile când apare o eroare.

Capcană frecventă:
- Confuzia dintre `"!"` (șir) și `'!'` (caracter `char`) la `push`.

Exemplu minim:
`let mut my_name = "Dave".to_string(); my_name.push('!'); println!("{}", my_name);`

Scor înțelegere: 10/10

## 1.2

1. Secțiunea explică structura de bază a unui program Rust.
2. Execuția începe în `fn main()`, iar codul este scris între acolade.
3. `println!` este folosit pentru a afișa text în consolă.
4. Instrucțiunile se termină, de obicei, cu `;`, ceea ce face codul mai clar.
5. Concluzia este că 1.2 te ajută să înțelegi forma minimă a unui program Rust.

Concepte-cheie:
- `//` pentru comentarii pe o linie.
- `/* ... */` pentru comentarii pe mai multe linii sau inline.
- Comentariile sunt pentru oameni, compilatorul le ignoră.

Capcană frecventă:
- Comentariile sunt utile, dar nu trebuie să înlocuiască denumiri bune de variabile.

Exemplu minim:
`let some_number = 100; // comentariu scurt`

Scor înțelegere: 10/10

## 1.3

1. Secțiunea introduce tipurile primitive: numere întregi, `char` și șiruri (`&str`/`String`).
2. Diferența-cheie este între tipuri semnate (`i*`) și nesemnate (`u*`), plus mărimea în biți.
3. `char` reprezintă un singur caracter Unicode, iar conversiile cu `as` trebuie făcute atent.
4. Pentru șiruri, `.len()` întoarce numărul de bytes, nu neapărat numărul de caractere.
5. Concluzia practică: pentru număr de caractere folosești `.chars().count()`, iar pentru bytes folosești `.len()`.

Concepte-cheie:
- `i*` = întregi semnați, `u*` = întregi nesemnați.
- `char` este un caracter Unicode (intern are dimensiune fixă).
- Conversiile explicite se fac cu `as`.

Capcană frecventă:
- `.len()` la string dă bytes, nu numărul de caractere vizibile.

Exemplu minim:
`println!("{} / {}", "안녕!".len(), "안녕!".chars().count());`

Scor înțelegere: 10/10

## 1.4

1. Secțiunea explică type inference: Rust ghicește de multe ori tipul corect fără să-l scrii explicit.
2. Pentru numere întregi, tipul implicit este de regulă `i32`, dacă nu specifici altceva.
3. Când vrei control clar, poți declara tipul cu `:` (ex: `let x: u8 = 10`) sau cu sufix (ex: `10u8`).
4. `_` în numere este doar pentru lizibilitate și este ignorat de compilator.
5. Concluzia practică: lasă Rust să infereze când e simplu, dar declară explicit tipul când ai nevoie de precizie.

Concepte-cheie:
- Rust inferează tipul când contextul e clar.
- Implicit, întregii fără tip explicit tind spre `i32`.
- Poți forța tipul cu `:` sau sufix (`10u8`, `100_i32`).

Capcană frecventă:
- Amestecul de tipuri diferite poate duce la erori de tip (mismatched types).

Exemplu minim:
`let a: u8 = 10; let b = 10u8; let c = 100_000_i32;`

Scor înțelegere: 10/10

## 1.5

1. Secțiunea introduce float-urile, adică numerele cu zecimale: `f32` și `f64`.
2. Un număr cu punct (`5.0`, `5.`) nu mai este integer, ci float.
3. Implicit, Rust tinde să folosească `f64` când nu specifici alt tip.
4. Rust nu permite direct operații între `f32` și `f64` fără conversie.
5. Concluzia practică: menține aceleași tipuri la calcule sau convertește explicit când ai nevoie.

Concepte-cheie:
- `f32` și `f64` au precizie/memorie diferită.
- Eroarea „mismatched types” apare când aduni tipuri diferite.
- Conversia explicită se poate face cu `as` (ex: `my_f32 as f64`).

Capcană frecventă:
- Amestecarea `f32` cu `f64` în aceeași expresie fără cast.

Exemplu minim:
`let a: f64 = 5.0; let b: f32 = 8.5; let c = a + b as f64;`

Scor înțelegere: 10/10

## 1.6

1. Secțiunea introduce exemplul clasic `Hello, world!` și explică afișarea cu `println!`.
2. Acoladele `{}` din `println!` pot afișa valori introduse direct sau returnate de funcții.
3. Funcțiile pot primi argumente și pot întoarce valori cu `->`.
4. În Rust, ultima expresie fără `;` este returnată implicit din funcție.
5. Concluzia practică: `println!`, argumentele și returnarea valorilor sunt baza pentru funcțiile următoare.

Concepte-cheie:
- `println!` este macro-ul standard pentru afișare.
- `fn nume(...) -> tip` definește o funcție care întoarce o valoare.
- Lipsa lui `;` pe ultima expresie permite return implicit.

Capcană frecventă:
- Dacă pui `;` la finalul expresiei returnate, funcția nu mai întoarce valoarea așteptată.

Exemplu minim:
`fn give_number() -> i32 { 8 } println!("{}", give_number());`

Scor înțelegere: 10/10

## 1.7

1. Secțiunea introduce declararea variabilelor cu `let` și utilizarea blocurilor de cod `{}`.
2. O variabilă trăiește doar în interiorul blocului în care este declarată (scope/lifetime).
3. `println!` permite atât forma cu argument după virgulă, cât și captura directă în șablon (`{my_number}`).
4. Un bloc poate returna o valoare dacă ultima expresie nu are `;`.
5. Concluzia practică: controlezi viața variabilelor și rezultatul blocurilor prin poziția codului și semicolon.

Concepte-cheie:
- `let` declară variabile.
- Scope-ul este delimitat de `{}`.
- Fără `;` la ultima expresie, blocul returnează o valoare.

Capcană frecventă:
- Dacă adaugi `;` la finalul expresiei din bloc, primești `()` în locul valorii așteptate.

Exemplu minim:
`let x = { let y = 8; y + 9 };`

Scor înțelegere: _/10

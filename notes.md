# Notițe carte Rust

## 1.1

1. Secțiunea introduce primul contact practic cu Rust.
2. Ideea principală este fluxul simplu: scriu cod, compilez și rulez.
3. Programul începe din `fn main()`, care este punctul de intrare.
4. Exemplul arată folosirea unui `String` mutabil și afișarea lui în consolă.
5. Concluzia este că 1.1 te familiarizează cu baza și cu mediul de lucru.

## 1.2

1. Secțiunea explică structura de bază a unui program Rust.
2. Execuția începe în `fn main()`, iar codul este scris între acolade.
3. `println!` este folosit pentru a afișa text în consolă.
4. Instrucțiunile se termină, de obicei, cu `;`, ceea ce face codul mai clar.
5. Concluzia este că 1.2 te ajută să înțelegi forma minimă a unui program Rust.

## 1.3

1. Secțiunea introduce tipurile primitive: numere întregi, `char` și șiruri (`&str`/`String`).
2. Diferența-cheie este între tipuri semnate (`i*`) și nesemnate (`u*`), plus mărimea în biți.
3. `char` reprezintă un singur caracter Unicode, iar conversiile cu `as` trebuie făcute atent.
4. Pentru șiruri, `.len()` întoarce numărul de bytes, nu neapărat numărul de caractere.
5. Concluzia practică: pentru număr de caractere folosești `.chars().count()`, iar pentru bytes folosești `.len()`.

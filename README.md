# Polygon Editor
Poniżej znajduje się instrukcja i założenia programu
## Instrukcja obsługi programu
Program operuje w 3 trybach widocznych na lewej stronie ekranu:
1. Dodawanie
2. Edycja
3. Reguły
### Tryb dodawania
Kolejne wierzchołki dodajemy za pomocą lewego przycisku myszy. Do zakończenia tworzenia poligonu używamy prawego przycisku myszy.
### Tryb Edycji
Zapomocą lewego przycisku myszy ruszamy wierzchołkami, krawędziami oraz poligonami. Aby poruszyć poligonem należy ruszać zaznaczonym środkiem ciężkości poligona. Za pomocą prawego przycisku myszy możemy usuwać wierzchołki i poligony oraz dzielić krawędzie na pół.
### Tryb reguł
Aby użyć trybu reguł należy najpierw zaznaczyć krawędź za pomocą lewego przycisku myszy. Wtedy za pomocą prawego przycisku myszy możemy kliknąć na inną krawędź aby utworzyć z nią relację równoległości. Po lewo znajduje się menu za pomocą którego można ustalić długość zaznaczonej krawędzi, ustalić że będzie ona stałej długości oraz usunąć istniejącą relacje z krawędzi.
### Przyciski
W lewym dolnym rogu znajdują się 3 przyciski. Ich kliknięcie odpowiada za:
1. Wyświetlenie pomocy
2. Wyświetlenie predefiniowanej sceny
3. Wygenerowania obecnej klatki za pomocą algorytmu Bresenhama.

## Założenia programu
Program zakłada, że w danym poligonie nie wszystkie krawędzie mają stale zdefiniowaną długość.

Algorytm relacji polega na zmianie wierzchołków krawędzi kolejno w obie strony od edytowanego wierzchołka dopóki nie trafimy na krawędź nie objętą żadną relacją. Jeśli krawędź jest objęta relacją stałej długości, odpowiedni wierzchołek jest przesuwany aby zachować tą długość i nie zmieniać obecnego kierunku krawędzi. Jeśli krawędź jest w relacji równoległości, poprawiamy drugą linie z relacji. Podczas działania tego algorytmu oznaczamy odwiedzone już krawędzi, co prowadzi do rzadkich przypadków brzegowych, gdy na linie ma wpływ wiele relacji, w których zachowanie nie jest zdefiniowane.

### Instrukcja uruchomienia
Poniżej znajduje się automatycznie wygenerowana instrukcja uruchomienia programu. Oprócz informacji zawartych poniżej wspomnieć należy o:
1. Instalacji nodejs i npm
2. instalacji rusta
3. instalacji paczki rust do webassembly za pomocą komendy ```cargo install wasm-pack```
## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

## What does each file do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

* The `tests` folder contains your Rust unit tests.

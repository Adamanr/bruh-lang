# bruh-lang

> Минималистичный эзотерический язык программирования с компилятором в WebAssembly.  
> A minimalist esoteric programming language with a WebAssembly compiler.

---

## Содержание / Table of Contents

- [Русский](#русский)
  - [Обзор](#обзор)
  - [Установка](#установка)
  - [Синтаксис языка](#синтаксис-языка)
  - [Использование](#использование)
  - [Примеры программ](#примеры-программ)
- [English](#english)
  - [Overview](#overview)
  - [Installation](#installation)
  - [Language Syntax](#language-syntax)
  - [Usage](#usage)
  - [Example Programs](#example-programs)

---

# Русский

## Обзор

**bruh-lang** — эзотерический язык в духе Brainfuck. У программы есть ровно три целочисленные переменные (`bruh.`, `bruh!`, `bruh?`), которые инициализируются нулём. Поддерживаются арифметика, присваивание, вывод и один уровень циклов.

Компилятор написан на Rust. Может как **интерпретировать** программы напрямую, так и **компилировать** их в портируемый WebAssembly-модуль (WASI), который запускается в любом wasm-рантайме.

## Установка

Требуется [Rust](https://rustup.rs/) (edition 2024).

```bash
git clone <repo>
cd bruhlang
cargo build --release
# Бинарь будет в target/release/bruh
```

Опционально: добавьте `target/release` в `$PATH`, чтобы вызывать `bruh` глобально.

Для запуска скомпилированных `.wasm`-файлов нужен [wasmtime](https://wasmtime.dev/):
```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Синтаксис языка

### Переменные

Программа имеет ровно три переменные. Все стартуют с нуля. Тип — знаковое 64-битное целое (`i64`), арифметика с wraparound.

| Токен   | Переменная |
|---------|-----------|
| `bruh.` | A         |
| `bruh!` | B         |
| `bruh?` | C         |

### Инструкции

Одна строка = одна инструкция. Пустые строки игнорируются. Комментарии начинаются с `#`.

#### Арифметика

| Синтаксис        | Действие              |
|------------------|-----------------------|
| `bruh. momentum` | A = A + 1             |
| `bruh. moment`   | A = A − 1             |
| `bruh. momentum bruh!` | A = A + B       |
| `bruh. moment bruh!`   | A = A − B       |
| `bruh. bruh!`    | A = B (присваивание)  |

*(Вместо `bruh.` и `bruh!` можно использовать любые из трёх переменных.)*

#### Вывод

| Синтаксис      | Действие                                          |
|----------------|---------------------------------------------------|
| `moment bruh.` | Напечатать A как десятичное число (знаковое i64)  |
| `momentum bruh.` | Напечатать A как символ (младший байт, 0–255)   |

Перевод строки не добавляется — программа сама управляет форматом вывода.

#### Циклы

Один уровень вложенности (вложенные циклы запрещены).

```
sound <лев> <прав>           # while лев != прав
sound <лев> momentum <прав>  # while лев > прав
sound <лев> moment <прав>    # while лев < прав
  ...тело...
effect
```

Условие проверяется **перед каждой итерацией** (`while`, не `do-while`). Сравнения знаковые.

### Ошибки компиляции

Компилятор сообщает об ошибках в формате:
```
bruh: error[Код] at line N: описание
```

| Код                | Причина                                          |
|--------------------|--------------------------------------------------|
| `UnknownToken`     | Нераспознанный токен                             |
| `TrailingTokens`   | Лишние токены в конце строки                     |
| `MalformedStatement` | Строка не соответствует ни одной инструкции    |
| `NestedLoop`       | `sound` внутри незакрытого цикла                 |
| `UnmatchedEffect`  | `effect` без соответствующего `sound`            |
| `UnterminatedLoop` | Конец файла внутри незакрытого цикла             |

## Использование

### Интерпретация

```bash
bruh run program.bruh
```

Запускает программу напрямую через tree-walking интерпретатор.

### Проверка синтаксиса

```bash
bruh check program.bruh
```

Запускает лексер + парсер + семантический анализ. Не выполняет программу. Код возврата `0` — всё ок, `1` — есть ошибки.

### Компиляция в WebAssembly

```bash
# Скомпилировать в бинарный .wasm (рядом с исходником)
bruh build program.bruh

# Указать путь к выходному файлу
bruh build program.bruh -o output.wasm

# Вывести читаемый WAT (текстовый формат WebAssembly)
bruh build program.bruh --emit wat

# Запустить скомпилированный модуль
wasmtime output.wasm
```

### Коды возврата

| Код | Значение                    |
|-----|-----------------------------|
| `0` | Успех                       |
| `1` | Ошибка компиляции           |
| `2` | Ошибка ввода-вывода / CLI   |

## Примеры программ

### Счётчик (`examples/count.bruh`)

Печатает `2`.

```
bruh? momentum    # C = 1
bruh? momentum    # C = 2

# while A < C: A++
sound bruh. moment bruh?
bruh. momentum
effect

moment bruh.      # напечатать A
```

```bash
bruh run examples/count.bruh
# → 2
```

### Обратный отсчёт (`examples/countdown.bruh`)

Печатает `321`.

```
# A = 3
bruh. momentum
bruh. momentum
bruh. momentum

# while A > C (C=0): напечатать A, A--
sound bruh. momentum bruh?
moment bruh.
bruh. moment
effect
```

```bash
bruh run examples/countdown.bruh
# → 321
```

### Буква A (`examples/letter_a.bruh`)

Печатает символ `A` (ASCII 65).

```
# 65 инкрементов, затем вывод как символ
bruh. momentum
bruh. momentum
# ... (всего 65 строк)
momentum bruh.
```

```bash
bruh run examples/letter_a.bruh
# → A
```

### Запуск через WebAssembly

```bash
bruh build examples/count.bruh -o count.wasm
wasmtime count.wasm
# → 2
```

---

# English

## Overview

**bruh-lang** is an esoteric programming language in the spirit of Brainfuck. A program has exactly three integer variables (`bruh.`, `bruh!`, `bruh?`), all initialised to zero. Supported operations: arithmetic, assignment, output, and a single level of loops.

The compiler is written in Rust. It can both **interpret** programs directly and **compile** them to a portable WebAssembly module (WASI) that runs in any wasm runtime.

## Installation

Requires [Rust](https://rustup.rs/) (edition 2024).

```bash
git clone <repo>
cd bruhlang
cargo build --release
# Binary will be at target/release/bruh
```

Optionally add `target/release` to your `$PATH` to use `bruh` globally.

To run compiled `.wasm` files, install [wasmtime](https://wasmtime.dev/):
```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Language Syntax

### Variables

A program has exactly three variables, all starting at zero. Values are signed 64-bit integers (`i64`) with wrapping arithmetic.

| Token   | Variable |
|---------|----------|
| `bruh.` | A        |
| `bruh!` | B        |
| `bruh?` | C        |

### Instructions

One line = one instruction. Empty lines are ignored. Comments start with `#`.

#### Arithmetic

| Syntax                 | Effect                     |
|------------------------|----------------------------|
| `bruh. momentum`       | A = A + 1                  |
| `bruh. moment`         | A = A − 1                  |
| `bruh. momentum bruh!` | A = A + B                  |
| `bruh. moment bruh!`   | A = A − B                  |
| `bruh. bruh!`          | A = B (assignment)         |

*(Any of the three variables can appear in place of `bruh.` and `bruh!`.)*

#### Output

| Syntax           | Effect                                             |
|------------------|----------------------------------------------------|
| `moment bruh.`   | Print A as a decimal number (signed i64)           |
| `momentum bruh.` | Print A as a character (least significant byte, 0–255) |

No newline is added — the program controls its own output format.

#### Loops

Single nesting level only (nested loops are a compile error).

```
sound <left> <right>           # while left != right
sound <left> momentum <right>  # while left > right
sound <left> moment <right>    # while left < right
  ...body...
effect
```

The condition is checked **before each iteration** (`while`, not `do-while`). Comparisons are signed.

### Compile Errors

The compiler reports errors in the format:
```
bruh: error[Code] at line N: description
```

| Code                 | Cause                                             |
|----------------------|---------------------------------------------------|
| `UnknownToken`       | Unrecognised token                                |
| `TrailingTokens`     | Extra tokens at the end of a line                 |
| `MalformedStatement` | Line does not match any valid instruction form    |
| `NestedLoop`         | `sound` encountered inside an unclosed loop       |
| `UnmatchedEffect`    | `effect` without a matching `sound`               |
| `UnterminatedLoop`   | End of file inside an unclosed loop               |

## Usage

### Interpret

```bash
bruh run program.bruh
```

Runs the program directly via a tree-walking interpreter.

### Syntax Check

```bash
bruh check program.bruh
```

Runs the lexer, parser, and semantic analyser without executing. Exit code `0` means success, `1` means errors were found.

### Compile to WebAssembly

```bash
# Compile to binary .wasm (next to the source file)
bruh build program.bruh

# Specify output path
bruh build program.bruh -o output.wasm

# Emit human-readable WAT (WebAssembly Text format)
bruh build program.bruh --emit wat

# Run the compiled module
wasmtime output.wasm
```

### Exit Codes

| Code | Meaning                    |
|------|----------------------------|
| `0`  | Success                    |
| `1`  | Compile error              |
| `2`  | I/O or CLI error           |

## Example Programs

### Counter (`examples/count.bruh`)

Prints `2`.

```
bruh? momentum    # C = 1
bruh? momentum    # C = 2

# while A < C: A++
sound bruh. moment bruh?
bruh. momentum
effect

moment bruh.      # print A
```

```bash
bruh run examples/count.bruh
# → 2
```

### Countdown (`examples/countdown.bruh`)

Prints `321`.

```
# A = 3
bruh. momentum
bruh. momentum
bruh. momentum

# while A > C (C=0): print A, A--
sound bruh. momentum bruh?
moment bruh.
bruh. moment
effect
```

```bash
bruh run examples/countdown.bruh
# → 321
```

### Letter A (`examples/letter_a.bruh`)

Prints the character `A` (ASCII 65).

```
# 65 increments, then print as character
bruh. momentum
bruh. momentum
# ... (65 lines total)
momentum bruh.
```

```bash
bruh run examples/letter_a.bruh
# → A
```

### Running via WebAssembly

```bash
bruh build examples/count.bruh -o count.wasm
wasmtime count.wasm
# → 2
```

---

## Project Structure

```
bruh-lang/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI (clap)
│   ├── lib.rs         # Public API
│   ├── token.rs       # Token types + positions
│   ├── lexer.rs       # Source text → token lines
│   ├── ast.rs         # AST node types
│   ├── parser.rs      # Token lines → raw statements
│   ├── sema.rs        # Semantic analysis, loop folding
│   ├── interp.rs      # Tree-walking interpreter
│   └── codegen/
│       ├── mod.rs
│       └── wasm.rs    # AST → WAT → WASM binary
├── examples/
│   ├── count.bruh
│   ├── countdown.bruh
│   └── letter_a.bruh
└── tests/
    ├── corpus/        # .bruh + .expected golden files
    └── integration.rs # Interpreter & WASM golden tests
```

## Running Tests

```bash
cargo test
```

All 13 tests cover the full corpus: positive cases (interpreter output matches `.expected` byte-for-byte) and negative cases (correct error codes). WASM tests run against `wasmtime` if it is in `$PATH`, and silently skip otherwise.

```bash
cargo clippy        # zero warnings
cargo build --release
```

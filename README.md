# dwn

Dawn (`dwn`) is the interpreter and bytecode compiler for the Dawn Programming Language.

# NOTE!: This project has been abandoned. A new project [`orion`](https://github.com/Solarcode-org/orion-lang) has been created in the [Solarcode Organisation](https://github.com/https://github.com/Solarcode-org)

## Installation options

- [Go to book](https://arnabrollin.github.io/dwn-book)
- [Latest Release](https://github.com/ArnabRollin/dwn/release/latest)
- [Releases](https://arnabrollin.github.io/dwn/releases)

## Examples

### User Greeting

```dwn
say "Hello!"
let name = (ask "What is your name? ")

say "Hello" name "!"
```

### Guessing Game

```dwn
let n = 1256

if (n == 1256) { ; test
 say "Hello! Welcome to The Guesser"
}

forever {
 let gs = (ask "Guess the number > ")
 let g = (int gs)

 if (g == n) {
  say "Congratulations! The number was" n
  break
 }

 if (g > n) {
  say "Too big!"
 }

 if (g < n) {
  say "Too small!"
 }
}
```

# dwn

`dwn` is the interpreter for the Dawn programming language.

## Examples

`code.dwn`

```dwn
let username = (ask "What is your name? ") ; Asking the user their name...

say "Hello" username "!" ; Greet the user...

let myname = "ExamplePerson"

say "My name is" myname "."

say "1 - 1 is" (1 - 1)
say "The sum of 123 and 678 is" (sum 123 678)
say "100 + 0.5 is" (100 + 0.5)
```

```console
% dwn run code.dwn

What is your name? ArnabRollin
Hello ArnabRollin!
My name is ExamplePerson.
1 - 1 is 0 
The sum of 123 and 678 is 801 
100 + 0.5 is 100.5
```

# Rustdoku
Hello this is my Sudoku solver in Rust !

## How could i install it ?
You must have `rust` installed !
Then you can clone this wounderful project.

## Then ? How could i solv my sudoku ?
### First, create your own sudoku
To create your sudoku you have to create a sudoku.txt at the root of the project.
He must be format with theses rules:
* empty case are represented by `.`
* there is 9 lines with 9 collumn
* the accepted characters are `123456789.`
* there is at least 17 numbers in your sudoku

Like this:
```
.89......
14.......
...2..5..
...9....1
.2..8..74
7..3..6.8
3.....1..
27..3...6
.1.....2.
```

### Then, launch my project !
To solve your sudoku use ```cargo run --release```

Care you can cross errors:
* GrillSize, this one mean your grill havn't 81 cases
* Multresult, here you have multiples solutions so give us 17 - or more - numbers in your sudoku
* ImpossibleGrill, here your sudoku havn't solutions
* And an other explicit error, mean that your sudoku got a strange character

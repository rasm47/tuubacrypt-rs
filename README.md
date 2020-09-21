# tuubacrypt-rs

a command-line interface for the "tuubacrypt" cipher

## Disclaimer

> Don't use this to encrypt sensitive information

The so called tuubacrypt cipher is a toy to play with and is not secure in 
any way.

## Build

``` sh
cargo build
```

## Example usage

``` sh
tuubacrypt-rs -e 'ENCRYPT ME'
```

``` sh
tuubacrypt-rs -d 'decrypt me 789'
```

``` sh
tuubacrypt-rs --encrypt --file nuclear_lauch_codes.txt --output encrypted.txt
```

for more details run

``` sh
tuubacrypt-rs --help
```

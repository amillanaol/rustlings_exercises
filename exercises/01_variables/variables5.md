# Ejercicio variables5

## Descripción

El script tiene como objetivo principal demostrar el uso de variables y
operaciones básicas. Sin embargo, presenta un error de compilación debido
a la reasignación de un valor a una variable declarada como `let`, que
por defecto es inmutable en Rust.

## Desglose paso a paso

1. **Declaración de la variable `number`**: La variable `number` se declara
   con el valor `"T-H-R-E-E"`, que es una cadena de texto.
2. **Impresión del valor de `number`**: Se imprime el valor de `number`,
   que muestra el texto `"Spell a number: T-H-R-E-E"`.
3. **Intento de reasignación de `number`**: Se intenta reasignar el
   valor `3` a la variable `number`, lo que causa un error de compilación
   porque `number` es inmutable por defecto.
4. **Operación con `number`**: Si la reasignación fuera posible,
   se realizaría la operación `number + 2` e imprimiría el resultado.

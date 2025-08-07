# Ejercicio variables4.rs

El script intenta declarar una variable `x` con el valor `3`, para luego
mostrar en consola, despues reasigna el valor a `5`, y, finalmente luego volver a imprimirla en pantalla. Sin embargo,el script contiene un error de compilación.

## Desglose Paso a Paso

1. **Declaración de la Variable `x`**: El script comienza declarando
   una variable `x` con el valor `3`.
2. **Impresión Inicial**: Se imprime el valor de `x`, que es `3`.
3. **Reasignación de `x`**: Se intenta reasignar el valor de `x` a `5`. **Aquí es
   donde ocurre el error de compilación**,porque en Rust, las variables
   son inmutables por defecto.
4. **Segunda Impresión**: Se intenta imprimir el nuevo valor de `x`, que debería
   ser `5` si la reasignación fuera exitosa.

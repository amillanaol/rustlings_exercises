# Ejercicio move_semantics1

## Descripción

Este script es un ejercicio de programación en Rust que está diseñado para enseñar
conceptos fundamentales sobre la **semántica de movimiento** (move semantics) y la
**mutabilidad** de variables. El código contiene un error de compilación intencional
que debe ser corregido para que funcione correctamente.

El ejercicio consiste en una función llamada `fill_vec` que debe recibir un vector
de números enteros, agregarle el número 88 al final, y retornar el vector modificado.
Sin embargo, tal como está escrito actualmente, el código no compilará debido a un
problema con la mutabilidad de la variable.

## Desglose paso a paso

**Función `fill_vec`:**

- Recibe un parámetro `vec` de tipo `Vec<i32>` **por valor** (no por referencia)
- Esto significa que la propiedad del vector se **mueve** desde el lugar donde se
llama la función hacia dentro de la función
- Dentro de la función, hace `let vec = vec;` que es redundante pero no cambia nada
- Intenta ejecutar `vec.push(88)` para agregar el número 88 al final del vector
- Finalmente retorna el vector modificado

**Función `main`:**

- Está vacía, solo tiene un comentario indicando que se puede experimentar ahí

**Test `move_semantics1`:**

- Crea un vector inicial con los valores `[22, 44, 66]`
- Llama a la función `fill_vec` pasándole este vector
- Espera que el resultado sea `[22, 44, 66, 88]` (el vector original más el 88 al
final)

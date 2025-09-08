# Ejercicio move_semantics3

## Descripción

Este ejercicio aborda un error común en Rust relacionado con la **mutabilidad**.
El código tiene una función que intenta modificar un vector añadiendo un elemento,
pero falla porque el parámetro no está marcado como mutable. El desafío es arreglar
el error sin agregar nuevas líneas de código.

## Desglose paso a paso

**1. El problema en la función `fill_vec`:**

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);  // ❌ ERROR: no se puede modificar vec
    vec
}
```

**2. ¿Por qué falla?**

- El parámetro `vec` es **inmutable** por defecto
- El método `push()` necesita modificar el vector
- Rust no permite modificar datos inmutables

**3. El test funciona correctamente:**

- Crea un vector con `[22, 44, 66]`
- Llama a `fill_vec()` esperando que añada 88
- Verifica que el resultado sea `[22, 44, 66, 88]`


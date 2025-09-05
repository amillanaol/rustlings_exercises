# Ejercicio move_semantics2

## Descripción

Este ejercicio trata sobre un concepto fundamental en Rust llamado **ownership**
(propiedad). El código presenta una función que modifica un vector, pero tiene un
problema: después de pasar el vector a la función, ya no podemos usarlo en el código
original porque Rust "mueve" la propiedad del dato.

## Desglose paso a paso

**1. La función `fill_vec`:**

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;  // Convertimos el parámetro en mutable
    vec.push(88);       // Añadimos el número 88 al final
    vec                 // Devolvemos el vector modificado
}
```

**2. El problema en el test:**

- Creamos `vec0` con valores `[22, 44, 66]`
- Llamamos a `fill_vec(vec0)` - aquí Rust "mueve" `vec0`, ya no podemos usarlo
- Intentamos usar `vec0` de nuevo en `assert_eq!` - ¡ERROR!

**3. ¿Por qué pasa esto?**
En Rust, cuando pasas una variable a una función, la **propiedad** se transfiere.
Es como dar tu juguete a un amigo: ya no es tuyo, es de él.

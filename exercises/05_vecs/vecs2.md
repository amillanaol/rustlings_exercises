# Ejercicio vecs2

## Descripción

Este script es un ejercicio que te enseña dos formas diferentes de transformar datos
en Rust: usando **bucles tradicionales** y usando **iteradores funcionales**. El
objetivo principal es multiplicar cada elemento de un slice (porción de array) por
2, pero el código tiene partes incompletas.

## Desglose paso a paso

### 1. Función `vec_loop` (Método tradicional)

```rust
fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    for element in input {
        // TODO: Multiplicar cada elemento por 2 y agregarlo al vector
    }
    output
}

```

- **Qué hace**: Recibe un slice de números enteros y debe devolver un nuevo vector
- **Cómo funciona**: Crea un vector vacío y usa un bucle `for` para recorrer cada
elemento
- **Estado actual**: Incompleta - falta la lógica para multiplicar por 2

### 2. Función `vec_map_example` (Ejemplo completo)

```rust
fn vec_map_example(input: &[i32]) -> Vec<i32> {
    input.iter().map(|element| element + 1).collect()
}
```

- **Qué hace**: Suma 1 a cada elemento del slice
- **Cómo funciona**:
  - `iter()` crea un iterador sobre los elementos
  - `map()` aplica una función a cada elemento (en este caso suma 1)
  - `collect()` recolecta los resultados en un nuevo vector
- **Estado**: Completa y funcional - sirve como ejemplo

### 3. Función `vec_map` (Método funcional incompleto)

```rust
fn vec_map(input: &[i32]) -> Vec<i32> {
    input.iter().map(|element| {
        // ??? - Aquí falta multiplicar por 2
    }).collect()
}
```

- **Qué debe hacer**: Lo mismo que `vec_loop` pero usando iteradores
- **Estado actual**: Incompleta - falta la operación de multiplicación

### 4. Pruebas unitarias

Las pruebas verifican que:

- `vec_loop([2, 4, 6, 8, 10])` devuelva `[4, 8, 12, 16, 20]`
- `vec_map_example([1, 2, 3])` devuelva `[2, 3, 4]`
- `vec_map([2, 4, 6, 8, 10])` devuelva `[4, 8, 12, 16, 20]`

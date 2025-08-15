# Ejercicio primitive_types4

## Descripción

El script escrito en Rust tiene como objetivo obtener un slice de un arreglo de enteros.
El slice debe contener todos los elementos del arreglo excepto el primero y el último.
El ejercicio se logra mediante la implementación de una prueba (`test`) que verifica
si el slice obtenido es igual a `[2, 3, 4]`.

## Paso a Paso

1. Se declara un arreglo de enteros `a` con los valores `[1, 2, 3, 4, 5]`.
2. Se utiliza la sintaxis de Rust para obtener un slice de `a`, 
   específicamente `&a[1..a.len() - 1]`.
   - `&a` indica que se quiere obtener una referencia a un slice de `a`.
   - `[1..a.len() - 1]` especifica el rango de índices que se quieren incluir en
     el slice.
   - El índice inicial es `1`, lo que significa que se excluye el primer 
     elemento (`1`).
   - `a.len() - 1` calcula el índice del último elemento y luego se resta `1` para
     excluir el último elemento.
3. Se utiliza la macro `assert_eq!` para verificar si el slice obtenido es igual
   a `[2, 3, 4]`.

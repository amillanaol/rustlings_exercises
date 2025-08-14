# Ejercicio if3

## Descripción

El script de Rust proporcionado define una función `animal_habitat` que determina
el hábitat de un animal según su nombre. La función utiliza una cadena de `if-else`
para asignar un identificador al animal y luego devuelve el hábitat correspondiente.
El script también incluye un conjunto de pruebas unitarias para verificar la corrección
de la función.

## Desglose paso a paso

1. La función `animal_habitat` toma un parámetro `animal` de tipo `&str`, que representa
   el nombre del animal.
2. Dentro de la función, se utiliza una cadena de `if-else` para asignar un identificador
   al animal según su nombre.
3. El identificador se utiliza luego en otra cadena de `if-else` para determinar
   el hábitat del animal.
4. La función devuelve el hábitat como un valor de tipo `&str`.
5. El script incluye un módulo de pruebas que verifica la corrección de la función
   `animal_habitat` para diferentes nombres de animales.

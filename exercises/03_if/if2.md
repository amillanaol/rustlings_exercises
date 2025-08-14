# Ejercicio if2

## Descripción

El código Rust proporcionado define una función `picky_eater` que intenta clasificar
la preferencia de un comensal por diferentes alimentos. La función actualmente tiene
un error de compilación debido a un retorno inválido. El objetivo es entender qué
hace el código y cómo se puede corregir para satisfacer los tests proporcionados.

## Desglose paso a paso

1. La función `picky_eater` toma un parámetro `food` de tipo `&str` y devuelve un
   valor de tipo `&str`.
2. Actualmente, la función solo devuelve "Yummy!" si el alimento es "strawberry",
   e intenta devolver `1` en otros casos, lo que causa un error de compilación.
3. Los tests indican que la función debería devolver "Yummy!" para "strawberry",
   "I guess I can eat that." para "potato", y "No thanks!" para cualquier otro alimento.

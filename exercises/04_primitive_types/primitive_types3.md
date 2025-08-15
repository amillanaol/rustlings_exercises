# Ejercicio primitive_types3

## Descripción

El script en Rust proporcionado es un ejercicio que busca crear un arreglo llamado
`a` con al menos 100 elementos. El programa verifica si el arreglo cumple con esta
condición y, dependiendo del resultado, imprime un mensaje y potencialmente entra
en pánico si el arreglo no es lo suficientemente grande.

## Desglose paso a paso

1. La función `main` es el punto de entrada del programa.
2. Se declara una variable `a` que se espera que sea un arreglo, pero su inicialización
   está comentada y pendiente de implementación (`// let a = ???`).
3. El programa verifica si la longitud del arreglo `a` es mayor o igual a 100 utilizando
   la condición `if a.len() >= 100`.
4. Si el arreglo tiene 100 o más elementos, imprime el mensaje "Wow, that's a big
   array!".
5. Si el arreglo tiene menos de 100 elementos, imprime "Meh, I eat arrays like that
   for breakfast." y luego entra en pánico con el mensaje "Array not big enough,
   more elements needed" debido a la llamada a `panic!`.

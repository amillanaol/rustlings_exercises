# Ejercicio struct1

## Descripción

El script tiene como objetivo definir estructuras para representar
colores utilizando diferentes tipos de estructuras: una estructura regular (`ColorRegularStruct`),
una estructura de tupla (`ColorTupleStruct`) y una estructura unitaria (`UnitStruct`).
El ejercicio se centra en completar las definiciones de estas estructuras y escribir
pruebas para demostrar su uso.

## Desglose paso a paso

1. **Definir la Estructura Regular (`ColorRegularStruct`)**: Esta estructura debe
tener campos para representar los componentes de un color, típicamente rojo (red),
verde (green) y azul (blue). Cada campo debe ser de un tipo adecuado para almacenar
valores de color, que generalmente son números enteros entre 0 y 255.

2. **Definir la Estructura de Tupla (`ColorTupleStruct`)**: Similar a la estructura
regular, pero definida como una tupla. Las estructuras de tupla en Rust son útiles
cuando se necesita un conjunto fijo de campos, pero no se requiere nombrarlos.

3. **Definir la Estructura Unitaria (`UnitStruct`)**: Esta estructura no tiene campos
y se utiliza cuando solo se necesita una unidad de tipo, sin datos asociados.

4. **Escribir Pruebas**:
   - Para `regular_structs`: Instanciar `ColorRegularStruct` con valores específicos
(por ejemplo, verde) y verificar que sus campos coincidan con los valores esperados.
   - Para `tuple_structs`: Instanciar `ColorTupleStruct` con valores específicos
y verificar sus campos.
   - Para `unit_structs`: Instanciar `UnitStruct`, convertirlo a una cadena utilizando
`{:?}` y verificar que la cadena resultante sea la esperada.

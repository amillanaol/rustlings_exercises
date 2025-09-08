// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let mi_vector = vec![10, 20, 30];
    println!("Vector original: {:?}", mi_vector);

    let vector_modificado = fill_vec(mi_vector);
    println!("Vector con 88 añadido: {:?}", vector_modificado);

    // Otro ejemplo
    let numeros = vec![1, 2, 3, 4, 5];
    let resultado = fill_vec(numeros);
    println!("Resultado: {:?}", resultado);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
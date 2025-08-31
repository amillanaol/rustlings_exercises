// FIXED: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.

    // Creamos un vector inicial
    let mi_vector = vec![22, 44, 66];
    println!("Vector original: {:?}", mi_vector);

    // Llamamos a la función fill_vec (recuerda que necesitas arreglar el error primero)
    let resultado = fill_vec(mi_vector);
    println!("Vector después de fill_vec: {:?}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}

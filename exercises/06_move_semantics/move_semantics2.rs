fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}



// Donc j'ai compris ce qu'il ce passe dans le code.

// on déclare la variables vec0 = un vecteur

// On déclara la variables vec1 = appelle vers la fonction fill_vec en passant en argument vec0.

// Ce qu'il ce passe c'est que quand je passe vec0 dans l'argument de la fonction fill_vec, vec0 pert le propriété du vecteur, tous ownership va vers vec1

// Donc quand on fais le test assert_eq! x 2 on ne peut pas afficher les deux variables car vec0 a perdu la propriété du vecteur 

// Donc on fais un .clone() pour cloner dans la stack cette variables.

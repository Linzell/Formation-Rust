pub fn ajouter_deux(x: i32) -> i32 {
  x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cela_fonctionne() {
        assert_eq!(4, ajouter_deux(2));
    }
}

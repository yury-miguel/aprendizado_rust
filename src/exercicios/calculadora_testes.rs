pub fn dividir(n1: i32, n2: i32) -> Result<i32, String> {
    if n2 != 0 {
        Ok(n1 / n2)
    } else{
        Err("Divisao por zero".to_string())
    }
}
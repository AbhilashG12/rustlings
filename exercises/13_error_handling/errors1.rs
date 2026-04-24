// TODO: This function refuses to generate text to be printed on a nametag if
fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        Err("Empty names aren't allowed".into())
    }
}

fn main() {
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into()),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            Err("Empty names aren't allowed".into()),
        );
    }
}

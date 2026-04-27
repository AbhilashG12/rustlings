//
// Right now, this function isn't handling the error case at all. What we want
// to do is: If we call the `total_cost` function on a string that is not a
// number, that function will return a `ParseIntError`. In that case, we want to
// immediately return that error from our function and not try to multiply and
// add.
//
// There are at least two ways to implement this that are both correct. But one
// is a lot shorter!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    let qty = match item_quantity.parse::<i32>(){
        Ok(num) => Ok(num*cost_per_item + processing_fee),
        Err(s) => Err(s)
    };

    qty

}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}

fn main() {}

#[defmt_test_macros::tests]
mod tests {
    #[init]
    fn first() {}

    #[init]
    fn second() {}
}

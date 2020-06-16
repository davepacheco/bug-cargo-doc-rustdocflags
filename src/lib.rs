/** a public function */
pub fn bar()
{
    foo()
}

/** a private function */
fn foo()
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

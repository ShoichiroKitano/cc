use cc::extruct::*;

#[test]
fn test_add1() {
    assert_eq!(
        extruct_argments("(aaaint a, int b)rest"),
        Ok(("rest", vec![("int", "a"), ("int", "b")]))
    );
}

use re2dfa_derive::Dfa;

//#[test]
fn foo() {
  #[derive(Dfa, Eq, PartialEq, Copy, Clone)]
  enum A {
    #[re = "hello"]
    X,
    #[re = "world"]
    #[eps]
    Y,
    #[eof]
    Z,
  }
//  let mut al = ALexer::new(b"hello");
//  assert_eq!(al.next().unwrap().piece, b"hello");
}
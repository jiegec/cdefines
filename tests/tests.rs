extern crate cdefines;

#[cdefines::preprocessor]
const INTEGER: &str = 
    "#define HEX 0x1234
    #define DEC	2333
    #define BIN	0b0110
    #define OCT	05404";

#[test]
fn test_ioctl() {
    assert_eq!(INTEGER_HEX, 0x1234);
    assert_eq!(INTEGER_DEC, 2333);
    assert_eq!(INTEGER_BIN, 0b0110);
    assert_eq!(INTEGER_OCT, 0o5404);
}
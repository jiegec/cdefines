extern crate cdefines;

#[cdefines::preprocessor]
const IOCTL: &str = 
    "#define TCGETS		0x5401
    #define TCSETS		0x5402
    #define TCSETSW		0x5403
    #define TCSETSF		0x5404";

#[test]
fn test_ioctl() {
    assert_eq!(IOCTL_TCGETS, 0x5401);
    assert_eq!(IOCTL_TCSETS, 0x5402);
    assert_eq!(IOCTL_TCSETSW, 0x5403);
    assert_eq!(IOCTL_TCSETSF, 0x5404);

    let value = IOCTL::TCGETS;
    assert_eq!(value as usize, 0x5401);
}
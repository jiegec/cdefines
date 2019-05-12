# cdefines

Turn `#define` C codes into Rust constants.

## How to use:

Add to code:

```
extern crate cdefines;

#[cdefines::preprocessor]
const IOCTL: &str = 
    "#define TCGETS		0x5401
    #define TCSETS		0x5402
    #define TCSETSW		0x5403
    #define TCSETSF		0x5404";
```

It gets translated to:

```
const IOCTL_TCGETS: usize = 0x5401;
// ...
enum IOCTL {
    TCGETS = 0x5401,
    // ...
}
```

## What is supported



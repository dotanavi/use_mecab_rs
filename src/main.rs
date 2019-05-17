use std::ptr;
use std::ffi::{CStr, CString};

enum Mecab {}
type MecabPtr = *const Mecab;
type CharPtr = *const libc::c_char;

#[link(name = "mecab")]
extern "C" {
    // fn mecab_new(argc: libc::c_int, argv: *const CharPtr) -> MecabPtr;
    fn mecab_new2(arg: CharPtr) -> MecabPtr;
    fn mecab_destroy(mecab: MecabPtr);

    fn mecab_strerror(mecab: MecabPtr) -> CharPtr;

    fn mecab_sparse_tostr(mecab: MecabPtr, str: CharPtr) -> CharPtr;
}

fn print(s: CharPtr) {
    let s = unsafe { CStr::from_ptr(s) };
    match s.to_str() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}

unsafe fn main2(mecab: MecabPtr) {
    let input = CString::new("太郎は次郎が持っている本を花子に渡した。").unwrap();
    let result = mecab_sparse_tostr(mecab, input.as_ptr());
    if result == ptr::null() {
        let err_msg = mecab_strerror(mecab);
        print(err_msg);
        return;
    }
    print(result);
}

fn main() {
    unsafe {
        let input = CString::new("-d /usr/local/lib/mecab/dic/ipadic").unwrap();
        let mecab = mecab_new2(input.as_ptr());
        if mecab == ptr::null() {
            println!("mecabのインスタンスの作成に失敗しました");
            print(mecab_strerror(ptr::null()));
            return;
        }
        main2(mecab);
        mecab_destroy(mecab);
    }
}

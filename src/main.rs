

#[repr(C)]
struct Mecab(*const libc::c_void);

#[link(name = "mecab")]
extern "C" {
    fn mecab_new(argc: libc::c_int, argv: *const *const libc::c_char) -> Mecab;
    fn mecab_destroy(mecab: Mecab);
}

fn main() {
    unsafe {
        let mecab = mecab_new(0, std::ptr::null());
        println!("Hello, world!");
        mecab_destroy(mecab);
    }
}

fn main() {
    cc::Build::new()
        .files(&[
            "xdiff/xdiffi.c",
            "xdiff/xemit.c",
            "xdiff/xhistogram.c",
            "xdiff/xpatience.c",
            "xdiff/xprepare.c",
            "xdiff/xutils.c",
        ])
        .include("xdiff")
        .flag("-Wno-unused-parameter")
        .compile("xdiff");
}

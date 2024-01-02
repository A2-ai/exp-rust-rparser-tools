// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_rparsertools_extendr(void *dll);

void R_init_rparsertools(void *dll) {
    R_init_rparsertools_extendr(dll);
}

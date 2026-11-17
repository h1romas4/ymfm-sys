/* Bare wasm has no stderr or process to terminate; fatal errors trap instead. */
__attribute__((noreturn)) void abort(void) {
    __builtin_trap();
}

__attribute__((noreturn)) void __assert2(
    const char *file, int line, const char *function, const char *expression) {
    (void)file;
    (void)line;
    (void)function;
    (void)expression;
    __builtin_trap();
}

__attribute__((noreturn)) void abort_message(const char *format, ...) {
    (void)format;
    __builtin_trap();
}

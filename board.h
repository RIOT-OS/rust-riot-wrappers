/** A board file for Rust's board-generic riot-sys crate that overrides
 * everything that a board could do differently to be nonsentical. That would
 * never build as a C file, but hopefully things like UART_STDIO_DEV are
 * irrelevant to the Rust API anyway. */

#define UART_STDIO_DEV irrelevant
#define UART_STDIO_BAUDRATE irrelevant
#define UART_STDIO_RX_BUFSIZE irrelevant

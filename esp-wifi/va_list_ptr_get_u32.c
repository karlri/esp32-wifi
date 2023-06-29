#include <stdarg.h>

// compile with xtensa-esp32-elf-gcc -c va_list_ptr_get_u32.c -o lib_va_list_ptr_get_u32.a
int va_list_ptr_get_u32(va_list *list)
{
    return va_arg(*list, int);
}

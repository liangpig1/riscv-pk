#include <stdint.h>

extern void mock_assert(const int result, const char* const expression,
                            const char * const file, const int line);

void __wrap_printm(const char* s, ...)
{
  return;
}

void __wrap_poweroff(uint16_t code)
{
  mock_assert(0, "poweroff", __FILE__, __LINE__);
}

uint32_t __wrap_mcall_console_putchar(uint8_t ch)
{
  return 0;
}

long __wrap_disabled_hart_mask;
uint64_t __wrap_hart_mask;

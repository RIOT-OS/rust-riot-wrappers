#include <shell.h>
#include <thread.h>
#include <stdio_base.h>
#include <periph/i2c.h>
#include <net/gnrc.h>
#include <saul.h>
#include <saul_reg.h>

#ifndef I2C_COUNT
#define I2C_COUNT 0
#endif

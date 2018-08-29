#include <shell.h>
#include <thread.h>
#include <stdio_base.h>
#include <periph/i2c.h>
#include <net/gnrc.h>
#include <net/gnrc/udp.h>
#include <net/gnrc/pktbuf.h>
#include <net/gnrc/ipv6.h>
#include <net/gnrc/nettype.h>
#include <net/gnrc/netapi.h>
#include <saul.h>
#include <saul_reg.h>

#ifndef I2C_COUNT
#define I2C_COUNT 0
#endif

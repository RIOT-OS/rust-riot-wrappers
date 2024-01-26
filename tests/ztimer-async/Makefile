# name of your application
APPLICATION = riot-wrappers-test-ztimer_async
BOARD ?= native
APPLICATION_RUST_MODULE = riot_wrappers_test_ztimer_async
BASELIBS += $(APPLICATION_RUST_MODULE).module
FEATURES_REQUIRED += rust_target

USEMODULE += ztimer_usec
USEMODULE += ztimer_msec

# needed by embassy-executor-riot
USEMODULE += core_thread_flags

include $(RIOTBASE)/Makefile.include

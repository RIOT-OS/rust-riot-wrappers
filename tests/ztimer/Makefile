# name of your application
APPLICATION = riot-wrappers-test-ztimer
BOARD ?= native
APPLICATION_RUST_MODULE = riot_wrappers_test_ztimer
BASELIBS += $(APPLICATION_RUST_MODULE).module
FEATURES_REQUIRED += rust_target

USEMODULE += ztimer_usec
USEMODULE += ztimer_msec

include $(RIOTBASE)/Makefile.include

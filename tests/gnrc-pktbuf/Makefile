# name of your application
APPLICATION = riot-wrappers-test-gnrc-pktbuf
APPLICATION_RUST_MODULE = riot_wrappers_test_gnrc_pktbuf
BASELIBS += $(APPLICATION_RUST_MODULE).module
FEATURES_REQUIRED += rust_target

# FIXME: That's not a good criterion, gnrc_pktbuf can well stand alone
USEMODULE += gnrc
USEMODULE += gnrc_pktbuf

# thus we get `gnrc_pktbuf_is_empty` and `_is_sane`
CFLAGS += -DTEST_SUITES

include $(RIOTBASE)/Makefile.include

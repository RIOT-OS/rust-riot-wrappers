APPLICATION = riot-wrappers-test-uart
APPLICATION_RUST_MODULE = riot_wrappers_test_uart
BASELIBS += $(APPLICATION_RUST_MODULE).module
FEATURES_REQUIRED += rust_target
FEATURES_REQUIRED += periph_uart
# We don't use them, but still enable them to increase build test coverage
FEATURES_OPTIONAL += periph_uart_modecfg
FEATURES_OPTIONAL += periph_uart_reconfigure

include $(RIOTBASE)/Makefile.include

This crate contains wrappers around the RIOT C API exposed by riot-sys and
makes an attempt to provide idiomatic Rust wrappers (eg. implementing
embedded-hal for peripherals, implementing fmt::Write for stdio) around those.

On the use of bindgen
---------------------

This module uses a RIOT_EXPANDED_HEADERS environment variable as does riot-sys,
and decides from it which modules to enable: If MODULE_SAUL is not set, the
saul module will not be built in. Bindgen is used only to extract those
settings, not to generate code (that's for riot-sys to do).

This makes things very auto-magical, and I'm not yet sure whether that's the
best way for things to be. The Cargo way would be that the crate using
riot-wrappers actively enables some features in riot-wrappers, which then pulls
in features in riot-sys -- but riot-sys can't enable RIOT modules any more as
RIOT is already configured. The RIOT way would be to enable the modules the
application needs in the Makefile (possibly with dependencies pulling others
in), but the crate not being a module makes that hard.

This automagic way is convenient now; later iterations might be more explicit
and profit from better integration.

This crate contains wrappers around the `RIOT Operating System`_'s C API exposed by riot-sys and
makes an attempt to provide idiomatic Rust wrappers (eg. implementing
embedded-hal for peripherals, implementing fmt::Write for stdio) around those.

The `crate documentation`_ outlines which modules are available, and which
other crates' traits they implement.

For practical use and an introduction, see the examples_.

.. _`RIOT Operating System`: https://riot-os.org/
.. _`crate documentation`: https://docs.rs/riot-wrappers/
.. _examples: https://gitlab.com/etonomy/riot-examples/

Library and run-time components
-------------------------------

The riot-wrappers crate tries to stay out of the way by default to enable
various types of applications (ie. not only "Rust application running atop
RIOT", but also "RIOT module / driver implemented in Rust" or others).

To facilitate what is currently the best explored use case ("Rust application
running atop RIOT"), applications can use the ``main!`` macro to wrap a regular
Rust function like ``fn main() -> ()`` into a function that's exported with
proper name and signature to serve as ``main`` function in RIOT.

When that is used, it also makes sense to enable the ``set_panic_handler``
feature. It implements a panic handler that outputs the panic message to RIOT's
standard output, and puts the affected thread to sleep permanently.  (There is
no unwinding or similar; threads in RIOT are not really expected to terminate
and be restarted).

With such a main function and panic handler, a Rust crate can be built as a
static library and linked as a part of the RIOT build process without the need
for application specific C code.

See the riot-examples_ repository for complete setup examples.

.. _riot-examples: https://gitlab.com/etonomy/riot-examples

On the use of bindgen
---------------------

This module uses a RIOT_CFLAGS environment variable as does riot-sys,
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

License
-------

This crate is licensed under the same terms as of the LGPL 2.1, following the
license terms of the RIOT Operating System.

It is maintained by Christian M. Amsüss <ca@etonomy.org> as part of the etonomy
project, see <https://etonomy.org/>.

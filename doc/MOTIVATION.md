## Motivation for creating this project

While The Fuck is certainly magnificient, it does have a fatal flaw: it is
written in Python. With all due respect, Python is slow and this does harm the
user experience in two ways:

- It creates a perceivable and annoying slowdown during the shell startup,
  because it is written in Python.
- The fixes themselves can be rather slow.

On top of that, sometimes system-wide Python packages just break. In fact, this
happened to me while I've been writing this page and trying to do benchmarks.

The intention behind `fixit` is to solve this by the re-write in a natively
compiled language. Namely, in Rust. This removes the overhead of the Python
interpreter and opens up the potential to search for fixes utilizing all of the
CPU cores.

### On "instant mode"

The Fuck has a feature called "instant mode" where it wraps around your shell to
log output and read it instead of re-running the previous command. While this
approach is certainly useful and has the benefit of being available on every
terminal emulator locally, over SSH remotely, and without any additional
terminal configuration, I am not a big fan of it. Going this way can mess with
your shell output and creates a mess of nested processes. I am not totally
against it and would totally love if someone implements it for me, but for this
application the preferred way is to integrate with the terminal emulator API if
such option is available. The ones that I'm aware of with appropriate APIs are
WezTerm, kitty and iTerm2 .This way you do not create an additional layer
between a shell and a user and the fallback to just re-running a command is very
straightforward without editing shell configuration files.

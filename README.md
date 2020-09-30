# rsyes - YES implementation in Rust

Naive implementation of coreutils `yes` tool in Rust. Implements
some buffering as well, trying to match the "real thing"'s performance.
For some reason the debug build is a little faster:

    $ timeout 1 /bin/yes > yes.txt
    $ timeout 1 target/release/rsyes > rsyes.txt
    $ timeout 1 target/debug/rsyes > rsyes_debug.txt
    $ ls -ltrh *.txt
    -rw-rw-r--. 1 rytis rytis 861M Sep 30 10:16 yes.txt
    -rw-rw-r--. 1 rytis rytis 829M Sep 30 10:16 rsyes.txt
    -rw-rw-r--. 1 rytis rytis 843M Sep 30 10:16 rsyes_debug.txt

For comparison, the first implementation without internal buffering:

    $ timeout 1 /bin/yes > yes.txt
    $ timeout 1 target/release/rsyes > rsyes.txt
    $ ls -ltrh *.txt
    -rw-rw-r--. 1 rytis rytis 866M Sep 29 21:57 yes.txt
    -rw-rw-r--. 1 rytis rytis 1.4M Sep 29 21:57 rsyes.txt


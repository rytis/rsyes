# rsyes - YES implementation in Rust

Naive implementation of coreutils `yes` tool in Rust. Implements
some buffering as well, trying to match the "real thing"'s performance:

    $ timeout 1 /bin/yes > yes.txt
    $ timeout 1 target/release/rsyes > rsyes.txt
    $ ls -ltrh *.txt
    -rw-rw-r--. 1 rytis rytis 835M Sep 30 09:59 yes.txt
    -rw-rw-r--. 1 rytis rytis 769M Sep 30 09:59 rsyes.txt

For comparison, the first implementation without internal buffering:

    $ timeout 1 /bin/yes > yes.txt
    $ timeout 1 target/release/rsyes > rsyes.txt
    $ ls -ltrh *.txt
    -rw-rw-r--. 1 rytis rytis 866M Sep 29 21:57 yes.txt
    -rw-rw-r--. 1 rytis rytis 1.4M Sep 29 21:57 rsyes.txt


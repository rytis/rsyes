# rsyes - YES implementation in Rust

At the moment this is a somewhat bleak implementation
of the coreutils `yes` tool:

    $ timeout 1 /bin/yes > yes.txt
    $ timeout 1 target/release/rsyes > rsyes.txt
    $ ls -ltrh *.txt
    -rw-rw-r--. 1 rytis rytis 866M Sep 29 21:57 yes.txt
    -rw-rw-r--. 1 rytis rytis 1.4M Sep 29 21:57 rsyes.txt


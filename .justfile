set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]
set unstable := true

fmt:
    just --unstable --fmt
    treefmt --clear-cache --fail-on-change

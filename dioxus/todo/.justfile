set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]
set unstable := true

serve:
    just desktop

desktop:
    dx serve --package desktop

mobile:
    dx serve --package mobile --features web

web:
    dx serve --package web

fmt:
    just --unstable --fmt
    treefmt --clear-cache --fail-on-change

set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]
set unstable := true
set fallback := true

serve: desktop

build:
    dx build --package desktop
    dx build --package mobile
    dx build --package web

clean:
    dx clean --package desktop
    dx clean --package mobile
    dx clean --package web

desktop:
    dx serve --package desktop

mobile:
    dx serve --package mobile --features web

web:
    dx serve --package web

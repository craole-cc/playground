set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]
set unstable := true
set fallback := true

serve:
  dx serve

web:
  dx serve --platform web

desktop:
  dx serve --platform desktop

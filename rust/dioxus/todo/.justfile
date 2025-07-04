set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]
set unstable := true
set fallback := true

#{ List of packages to manage }

packages := "desktop mobile web"

# serve: desktop
serve:
  dx serve

# { Run dx command for all packages }
_dx_all command:
    for pkg in {{ packages }}; do dx {{ command }} --package $pkg; done

# { Build all packages }
build:
    just _dx_all build

# { Clean all packages }
clean:
    just _dx_all clean

# { Check if dx serve is running }
_check_dx_serving:
    #!/usr/bin/env sh
    if command -v tasklist >/dev/null 2>&1; then
        # Windows
        tasklist | findstr dx >/dev/null 2>&1
    else
        # Unix-like systems
        pgrep -f "dx serve" >/dev/null 2>&1
    fi

# { Define the serve function }
_serve package:
    #!/usr/bin/env sh
    just _check_dx_serving 2>/dev/null
    if [ $? -eq 0 ]; then
        echo "⚠️  dx serve is already running! Check your other terminals."
    else
        dx serve --package {{ package }}
    fi

# { Generate serve commands for each package }
[no-cd]
_generate_serve_commands:
    #!/usr/bin/env sh
    for pkg in {{ packages }}; do
        echo "$pkg:";
        echo "    just _serve $pkg";
    done

# { Serve commands for each package }
desktop:
    @just _serve desktop

mobile:
    @just _serve mobile --features web

web:
    @just _serve web

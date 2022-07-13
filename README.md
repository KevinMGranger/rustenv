# rustenv

Various environment related tools in rust.

# dhallenv

Convert a dhall configuration to the equivalent dotenv file.

# fishdotenv

Convert a dotenv file to `set` commands `source`-able by the fish shell.

`fishdotenv | source` will do what you want. See `--help` for more details.

You can add the dotenv status to your prompt if you want. Here's a sample:

```fish
if which fishdotenv > /dev/null 2>&1
    fishdotenv -c > /dev/null 2>&1
    switch $status
        case 0 # in sync
            set dotenv_status "✔️"
        case 3 # file missing
            set dotenv_status "?"
        case 4 # out of sync
            set dotenv_status (echo (set_color --bold red) "OUT OF SYNC" (set_color normal))
        case '*' # some other error
            set dotenv_status (echo (set_color red) "e$status" (set_color normal))
    end
else
    set dotenv_status "fishdotenv not installed"
end

# later, in the prompt itself

printf "🇪 %s" $dotenv_status
```

To have your .env file loaded whenever you change dirs (but only if the dir is trusted!),
here are some ideas:

~/.config/fish/conf.d/dotenv.fish
```fish
function dotenv_on_trusted_dir --on-variable PWD --description "update variables from .env if the dir is trusted"
    if not set -q trusted_dotenv_dirs;
        set --universal --path trusted_dotenv_dirs
    end
    if which fishdotenv >/dev/null 2>&1; and contains -- $PWD $trusted_dotenv_dirs
        fishdotenv | source
    end
end

dotenv_on_trusted_dir
```

~/.config/fish/functions/add_to_trusted_dotenv_dirs.fish
```fish
function add_to_trusted_dotenv_dirs --description "Add the current path or given paths to the list of trusted dotenv dirs"
    if test (count $argv) -eq 0;
        set --universal --path --append trusted_dotenv_dirs $PWD
    else
        set --universal --path --append trusted_dotenv_dirs $argv
    end
end
```

These may be printable in a future version of the program.
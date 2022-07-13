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
Generate command alias for different shell

# Usage:
`gen-alias.exe <ALIAS FILE> <SHELL>`

Where ALIAS FILE is the plain text file explained below.

Available shells:
- pwsh
- bash
- fish

# Integration

## Powershell
Add below line in the profile (Get its location by `echo $profile`)
`Invoke-Expression (& { gen-alias <ALIAS FILE> pwsh | Out-String})`

## Bash
Add below line in the profile, typically `~/.bashrc`
`eval "$(get-alias <ALIAS FILE> bash)"`

# Fish
Add below line in the profile, typically `~/.config/fish/config.fish`
`gen-alias alias.txt fish | source`

# Alias file
A utf-8 encoded file contains multiple line of alias.  
Each line has format like `<alias name>[:<supported shell 1>[,<supported shell 2>]=<commands>]`  
Example:
```
gclr=git checkout . && git clean -fd
ga:bash,fish=git add @
```
The `@` sign in the command string will be replaced to the parameter of the alias, like `@args` in powershell, or `$@` in bash. If no `@` sign is given in the value, I will append it at the end of the command.  

|symbols|meaning|powershell|bash|
|---|---|---|---|
|`@`|Expand parameters of alias.<br>If the value doesn't has `@` at all, the tooll will append it at the end automatically|`@args`|`$@`|
|`$1`, `$2`, ...|Parameters of the command|`$args[0]`|`$0`|

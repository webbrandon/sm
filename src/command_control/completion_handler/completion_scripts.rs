pub struct CompletionScript { }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    _sm() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            sm)
                cmd="sm"
                ;;
            
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            help)
                cmd+="__help"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        sm)
            opts=" -v -h -i -m -o  --verbose --help --input --mixin --output   completions"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --input)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --mixin)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        sm__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        sm__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        sm__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        sm__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        sm__completions__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        sm__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        sm__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _sm -o bashdefault -o default sm

    "#);
        }

        pub fn fish() {
            println!("{}",r#"
    complete -c sm -n "__fish_use_subcommand" -s i -l input -d 'Base file path.'
complete -c sm -n "__fish_use_subcommand" -s m -l mixin -d 'File path.'
complete -c sm -n "__fish_use_subcommand" -s o -l output -d 'Output to file and leave base file as is'
complete -c sm -n "__fish_use_subcommand" -s v -l verbose -d 'Enable verbose logging.'
complete -c sm -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_use_subcommand" -f -a "completions" -d 'Completion scripts for various shells.'
complete -c sm -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c sm -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script.'
complete -c sm -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script.'
complete -c sm -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script.'
complete -c sm -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script.'
complete -c sm -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script.'
complete -c sm -n "__fish_seen_subcommand_from completions" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c sm -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c sm -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c sm -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c sm -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c sm -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'
complete -c sm -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c sm -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

    "#);
        }

        pub fn zsh() {
            println!("{}",r#"
    #compdef sm

autoload -U is-at-least

_sm() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-i+[Base file path.]' \
'--input=[Base file path.]' \
'-m+[File path.]' \
'--mixin=[File path.]' \
'-o+[Output to file and leave base file as is]' \
'--output=[Output to file and leave base file as is]' \
'-v[Enable verbose logging.]' \
'--verbose[Enable verbose logging.]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
":: :_sm_commands" \
"*::: :->sm" \
&& ret=0
    case $state in
    (sm)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:sm-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_sm__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:sm-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script.:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script.:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script.:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script.:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script.:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_sm_commands] )) ||
_sm_commands() {
    local commands; commands=(
        "completions:Completion scripts for various shells." \
    )
    _describe -t commands 'sm commands' commands "$@"
}
(( $+functions[_sm__completions__bash_commands] )) ||
_sm__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sm completions bash commands' commands "$@"
}
(( $+functions[_sm__completions_commands] )) ||
_sm__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script." \
"fish:Fish completion script." \
"zsh:Zsh completion script." \
"powershell:PowerShell completion script." \
"elvish:Elvish completion script." \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'sm completions commands' commands "$@"
}
(( $+functions[_sm__completions__elvish_commands] )) ||
_sm__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sm completions elvish commands' commands "$@"
}
(( $+functions[_sm__completions__fish_commands] )) ||
_sm__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sm completions fish commands' commands "$@"
}
(( $+functions[_sm__completions__help_commands] )) ||
_sm__completions__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sm completions help commands' commands "$@"
}
(( $+functions[_sm__completions__powershell_commands] )) ||
_sm__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sm completions powershell commands' commands "$@"
}
(( $+functions[_sm__completions__zsh_commands] )) ||
_sm__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'sm completions zsh commands' commands "$@"
}

_sm "$@"
    "#);
        }

        pub fn powershell() {
            println!("{}",r#"
    
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'sm' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'sm'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'sm' {
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'Base file path.')
            [CompletionResult]::new('--input', 'input', [CompletionResultType]::ParameterName, 'Base file path.')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'File path.')
            [CompletionResult]::new('--mixin', 'mixin', [CompletionResultType]::ParameterName, 'File path.')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Output to file and leave base file as is')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Output to file and leave base file as is')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging.')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells.')
            break
        }
        'sm;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script.')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script.')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script.')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script.')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script.')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'sm;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'sm;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'sm;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'sm;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'sm;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'sm;completions;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

    "#);
        }

        pub fn elvish() {
            println!("{}",r#"
            
edit:completion:arg-completer[sm] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'sm'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'sm'= {
            cand -i 'Base file path.'
            cand --input 'Base file path.'
            cand -m 'File path.'
            cand --mixin 'File path.'
            cand -o 'Output to file and leave base file as is'
            cand --output 'Output to file and leave base file as is'
            cand -v 'Enable verbose logging.'
            cand --verbose 'Enable verbose logging.'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand completions 'Completion scripts for various shells.'
        }
        &'sm;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script.'
            cand fish 'Fish completion script.'
            cand zsh 'Zsh completion script.'
            cand powershell 'PowerShell completion script.'
            cand elvish 'Elvish completion script.'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'sm;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'sm;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'sm;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'sm;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'sm;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'sm;completions;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

    "#);
        }
    }
    
function __name__() {
    local previous_cmd
    previous_cmd="$(fc -ln -1)"
    local FIXIT_FNS
    FIXIT_FNS="$(
        declare -F | cut -d' ' -f3
        alias | cut -d'=' -f1
    )"
    export FIXIT_FNS
    local fixed_cmd
    fixed_cmd="$(fixit fix "$previous_cmd")"
    if [ "$fixed_cmd" != "" ]; then
        eval "$fixed_cmd"
	history -s "$fixed_cmd"
    fi
}

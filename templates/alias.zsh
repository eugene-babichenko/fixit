function {{name}}() {
    local previous_cmd="$(fc -ln -1)"
    local FIXIT_FNS="$(
        print -l ${(ok)functions}
        alias | cut -d'=' -f1
    )"
    typeset -x FIXIT_FNS
    local fixed_cmd="$(fixit fix "$previous_cmd")"
    if [ "$fixed_cmd" != "" ]; then
        eval "$fixed_cmd"
	print -s "$fixed_cmd"
    fi
}

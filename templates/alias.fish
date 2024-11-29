function {{name}} -d "Fix your previous command"
    set -l previous_cmd "$history[1]"
    set -lx FIXIT_FNS (
        functions | cut -d' ' -f1
        alias | cut -d' ' -f2
    )
    fixit fix "$previous_cmd" | read -l fixed_cmd
    if [ "$fixed_cmd" != "" ]
        commandline "$fixed_cmd"
        commandline -f execute
    end
end

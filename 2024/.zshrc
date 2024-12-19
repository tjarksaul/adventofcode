alias aoc="cargo run -- input.txt"
alias aoct="cargo run -- test.txt"
alias aot="cargo test"
function aoc-init {
    git_root=$(git rev-parse --show-toplevel)
    if [ -n "${2+1}" ]; then
        YEAR=$2
    else
        YEAR=$(date +%Y)
    fi
    if [ -n "${1+1}" ]; then
        DAY=$1
    else
        DAY=$(date +%d)
    fi

    mkdir -p ${git_root}/${YEAR}/
    cp -r ${git_root}/${YEAR}/template ${git_root}/${YEAR}/d${DAY}
    sed -i '' "s/\"template\"/\"d${DAY}\"/" ${git_root}/${YEAR}/d${DAY}/Cargo.toml
    echo "Created AoC ${YEAR}/d${DAY}"
}

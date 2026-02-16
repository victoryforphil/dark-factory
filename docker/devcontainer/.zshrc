export EDITOR=nvim
export VISUAL=nvim
export PAGER=less
export CLICOLOR=1
export TERM=xterm-256color

setopt HIST_IGNORE_DUPS
setopt SHARE_HISTORY

HISTFILE=${HOME}/.zsh_history
HISTSIZE=10000
SAVEHIST=10000

autoload -Uz colors && colors
PROMPT='%F{cyan}%n@%m%f %F{green}%~%f %# '

alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias gs='git status -sb'
alias moonc='moon run dark_core:check'
alias moond='moon run dark_core:dev'

FROM archlinux
RUN pacman -Syu && pacman -S base-devel rust

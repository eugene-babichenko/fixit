FROM archlinux
RUN pacman --noconfirm -Syu && pacman --noconfirm -S base-devel wget rust

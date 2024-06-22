FROM archlinux
RUN pacman --noconfirm -Syu && pacman --noconfirm -S base-devel rust

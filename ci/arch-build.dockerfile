FROM archlinux
RUN pacman --noconfirm -Syu && pacman --noconfirm -S base-devel wget rust
RUN chown -R nobody /project
USER nobody

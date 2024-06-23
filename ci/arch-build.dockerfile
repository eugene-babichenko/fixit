FROM archlinux
RUN pacman --noconfirm -Syu && pacman --noconfirm -S base-devel wget rust sudo
RUN useradd build
RUN passwd -d build
RUN echo "build ALL=(ALL) ALL" | tee -a /etc/sudoers

FROM archlinux@sha256:52dfbd5afb23a220d5ee92dafcb4c788d96c026ed04179890c23adef4c897fa0

LABEL org.opencontainers.image.title="ArchMaker Forge archiso builder"
LABEL org.opencontainers.image.description="mkarchiso with the pinned Arch Linux base image"
LABEL org.opencontainers.image.source="https://github.com/Alexendros/archmaker"

RUN pacman -Syu --noconfirm --needed archiso \
 && pacman -Scc --noconfirm

WORKDIR /work

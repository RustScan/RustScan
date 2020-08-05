FROM archlinux:latest

RUN yes | pacman -Syu
RUN yes | pacman -S fakeroot binutils sudo make gcc gettext awk git

WORKDIR /opt
RUN git clone https://aur.archlinux.org/yay-git.git

RUN useradd -m tecmint
RUN passwd -d tecmint
RUN printf 'tecmint ALL=(ALL) ALL\n' | tee -a /etc/sudoers

WORKDIR /opt/yay-git
RUN chown -R tecmint /opt/yay-git
USER tecmint
RUN makepkg -si --noconfirm

# AUR
RUN yay -S rustscan --noconfirm
RUN rustscan 127.0.0.1 -q -b 250 -t 1
RUN yay -R rustscan --noconfirm

# homebrew
RUN yes | /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
ENV PATH="/home/linuxbrew/.linuxbrew/bin:${PATH}"
RUN yes | brew tap brandonskerritt/rustscan
RUN yes | brew install rustscan
RUN rustscan 127.0.0.1 -q -b 250 -t 1
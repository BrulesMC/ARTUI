Requirements

This project requires asusctl and ryzenadj to function.

Install them from GitHub or via your distribution’s package manager if available:

asusctl: https://github.com/NeroReflex/asusctl

ryzenadj: https://github.com/FlyGoat/RyzenAdj

This project also requires Rust to compile.

Default config will be created in /home/$USER/.config as ar.json
use the default config as a guide, the docs for ryzenadj and asusctl may help

Warnings

⚠️ Sudoers modification
This project will modify your sudoers configuration to allow it to function. Review the changes carefully before proceeding.

⚠️ Stability notice
This project uses ryzenadj in a configuration that is sometimes unstable, depending on the power limits and current load.
Profile changes may cause system instability or crashes. This is a known limitation that can not be tested without access to non 6000 series proccessors.

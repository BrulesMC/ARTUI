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
Profile changes may cause system instability or crashes if the load is to heavy and the limits are low.
This appers to come from the system load balancing while shifting the min power.
This makes proccess move at the higher limit while they have no applied yet and the lower is still active.
This may be fixable but would require suspending proceess and may be system specific.
For now have load low when switching profiles to higher limits.

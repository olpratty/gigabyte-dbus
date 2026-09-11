# AORUS gigabyte-dbus

This fork of [tangalbert919/gigabyte-dbus](https://github.com/tangalbert919/gigabyte-dbus)
provides the privileged hardware-access service for the
[AORUS CoolerControl plugin](https://github.com/olpratty/aorus-coolercontrol-plugin).

`gigabyted` runs as root and writes the
[gigabyte-laptop-wmi](https://github.com/tangalbert919/gigabyte-laptop-wmi)
driver's sysfs attributes. The plugin runs unprivileged and sends fan-control
requests over the system D-Bus.

The supplied policy allows `cc-plugin-user` to call `SetFanMode` and
`SetFanSpeed`. This account is shared by CoolerControl's unprivileged plugins.

## Supported integration

Developed and hardware-tested on an AORUS 15P XD running custom Bazzite,
including CoolerControl 5.0, fixed duty, software curves and suspend/resume.

Battery charging and NVIDIA Dynamic Boost are outside this project's scope.
Other inherited methods are not granted to the plugin account and retain
their upstream error handling.

See [AORUS-INTEGRATION.md](AORUS-INTEGRATION.md) for method limits,
architecture and deployment details.

## Bazzite installation

The custom Bazzite AORUS image includes the daemon, policy, account definition
and enabled service. Update the image normally; do not install over its
immutable `/usr` using the Makefile.

## Build

Build as an ordinary user. On Fedora, the build requires Rust/Cargo, make,
gcc, pkgconf-pkg-config and systemd-devel.

```bash
make build
```

This builds only `gigabyted`, using Cargo.lock. The default output is
`target/release/gigabyted`; `make build DEBUG=1` builds a debug binary.

## Manual installation on a writable system

After building:

```bash
sudo make install
sudo systemd-sysusers /usr/lib/sysusers.d/gigabyted.conf
sudo systemctl daemon-reload
sudo systemctl reload dbus
sudo systemctl enable --now gigabyted.service
```

Create the plugin account before reloading the system-bus policy.
The kernel driver must be present for the daemon to start successfully.

The Makefile installs the same five files as the release archive. It does
not install the inherited udev rule or activate services automatically.
If upgrading a legacy manual installation, review and retire its old
gigabyted udev rule and any local service/policy overrides.

For package staging:

```bash
make install DESTDIR=/absolute/path/to/staging
```

## Manual uninstall

Stop dependent fan-control applications and return control to firmware first.

```bash
sudo systemctl disable --now gigabyted.service
sudo make uninstall
sudo systemctl daemon-reload
sudo systemctl reload dbus
```

Uninstall removes the installed files, not the shared `cc-plugin-user`
account or unrelated local configuration. For debug builds, use
`make install DEBUG=1` when installing.

## Upstream files

The inherited RPM spec, udev rule and interface.xml remain for reference.
They are not the installation path for this AORUS integration.
The current Rust implementation and AORUS integration document describe
the validated fan setters.

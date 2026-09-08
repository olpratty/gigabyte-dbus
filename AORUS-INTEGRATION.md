# AORUS integration

This fork of tangalbert919/gigabyte-dbus supports the custom Bazzite
AORUS 15P XD image and its CoolerControl device-service plugin.

## Architecture

- gigabyted runs as root and writes the kernel driver's sysfs attributes.
- The CoolerControl plugin runs as cc-plugin-user.
- The plugin reads fan RPM and PWM directly from read-only hwmon files.
- The packaged D-Bus policy permits cc-plugin-user to call only
  SetFanMode and SetFanSpeed.
- CoolerControl uses cc-plugin-user for unprivileged plugins; this
  permission therefore applies to other plugins running as that user too.

## Fan interface

Bus name: com.gigabyte.daemon
Object path: /com/gigabyte/Platform
Interface: com.gigabyte.Platform

SetFanMode accepts 0 (normal) and 5 (fixed).
SetFanSpeed accepts driver values 23 through 227, corresponding to
the plugin's tested 10–100% range on this laptop.

Successful setters return integer 0. Invalid arguments and write failures
return D-Bus errors. Hardware discovery or interface registration failure
causes daemon startup to fail.

These limits are specific to this integration and are not a general
compatibility claim for other Gigabyte laptops.

## Resume handling

The plugin re-establishes manual control when CoolerControl enables it:
normal mode, a two-second delay, then fixed mode and the requested duty.
This restores control after suspend/resume on the tested laptop.

## Deployment

The release archive contains:
- gigabyted
- gigabyted.service
- gigabyted.conf
- gigabyted.sysusers
- LICENSE

The Bazzite image must install the binary at /usr/bin/gigabyted,
install the service, system-bus policy and sysusers definition in their
vendor directories, and enable gigabyted.service.

The cc-plugin-user account must exist before the system bus loads its
policy. The supplied sysusers definition creates it.

The upstream Makefile, RPM spec and udev rules have not been adapted
for this release-archive deployment.

## Scope and limitations

Battery charging and NVIDIA Dynamic Boost are outside the project scope.
Unused upstream methods remain in the source and retain their original
error handling. They are not permitted for cc-plugin-user by this policy.

The fork's first release is 1.0.1, following the inherited upstream
v1.0.0 tag. Upstream Cargo.toml still reported 0.1.0 before this update.

# Geometry Sketchpad

A Cross-Platform Sketchpad written in Rust using [Piston](https://www.piston.rs) and [specs](https://slide-rs.github.io/specs/)

![screenshot.png](doc/images/screenshot_3.png)

(P.S. The screen shot is from [IMO 1959 Problem 5](https://artofproblemsolving.com/wiki/index.php/1959_IMO_Problems/Problem_5))

## How to build/run

Type

```
$ cargo run geopad-foundation --release
```

There are currently a `geopad-foundation` app which uses Piston as window driver, and a `geopad-win` as a port to windows. Simply change the above target to `geopad-win` to run this windows port. We will have linux, macos, electron and more applications in the future.

## How to use

See [interaction scheme](doc/interaction_scheme.md).
# Bevy Admob Example

This is an example repo showing how to integrate Google Admob with a Bevy project.

It is ultimately built off (a fork of) NiklasEi's [Bevy Game Template](https://github.com/niklasEi/bevy_game_template).

This README is WIP and should eventually point to a writeup on how this works + how to extend it.

## Instructions

To get this to work:

1) Install NiklasEi's xbuild fork:

```sh
cargo install --git https://github.com/niklasei/xbuild
```

2) Configure your android environment. (Use `x doctor` to see if there is anything critical missing.) See the [android environment setup section](#android-environment-setup) for more detail.

3) Ensure that you have a device visible on `x devices`. I prefer to plug an android device into my computer; alternatively, you can run an emulator. Either way, find the associated `host` value, e.g. `adb:XXXXXXXXXXXXXX`.

4) Run `x run -p snendev_ad_example_mobile --device <previous-host-value>`; once compiled, the app should run on your device.

5) A button should show on screen to call a test ad.

6) Once the ad is exited, it will return to the Bevy app. If the ad was redeemed, the text should change to say "Ad redeemed"; if the ad was cancelled, it will show the original "Load ad" text.

## Android Environment Setup



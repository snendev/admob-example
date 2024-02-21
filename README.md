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

4) Run `x run -p snendev_ad_example_mobile --device <host-code>`; once compiled, the app should run on your device.

5) A button should show on screen to call a test ad.

6) Once the ad is exited, it will return to the Bevy app. If the ad was redeemed, the text should change to say "Ad redeemed"; if the ad was cancelled, it will show the original "Load ad" text.

## How it works

The key Android class to be aware of is the `Activity` class. `Activity` is the base class for any application screens/routes/what-have-you that an app may want to run for the user. Bevy uses `winit` under the hood for window management, and when targeting the Android environment, winit does this by establishing a rendering context inside a `NativeActivity` using the `android-activity` crate.

`NativeActivity` is a special subclass of `Activity` that is designed for running native code, such as Rust code. We can think of the runtime established by `android-activity` like this:

```
_______________________________
|         JVM                 |
| ___________________________ |
| |   NativeActivity        | |
| | ______________________  | |
| | |  Bevy (Rust)       |  | |
| | |____________________|  | |
| |_________________________| |
|_____________________________|
```

Ultimately, the way we want to render admob ads is to switch to a different Activity which can render ads. (Alternatively, this could instead render admob NativeAds inside the Rust code, but this would imply the additional work of figuring out how to render these ads in native code, probably by integrating the appropriate video formats and more.) This requires understanding how to call Java code from the Rust side, which is the job of the Java Native Interface (jni). There is a `jni` crate which does exactly this!

`bevy_winit` exports a [constant](https://github.com/bevyengine/bevy/blob/main/crates/bevy_winit/src/lib.rs#L59) with a [method that returns a pointer to the activity](https://github.com/rust-mobile/android-activity/blob/c9faa9c44e04152863cc0ea9b6982065e0346ba6/android-activity/src/lib.rs#L499-L521). (Originally, I was also able to access a pointer to the JVM via `ndk-context`, which is used by various crates already in our dependencies, but this constant provides much more direct access to what we need. Shoutout to [this discord comment](https://discord.com/channels/691052431525675048/757316314845937785/1204499829988790313) which helped a lot.)

From here we need to call [`Activity.startActivity`](https://developer.android.com/reference/android/app/Activity#startActivity(android.content.Intent)), which requires an Android [`Intent`](https://developer.android.com/reference/android/content/Intent) that points to the target Activity class. Importantly, the JNI [does not know how to find non-system classes](https://developer.android.com/training/articles/perf-jni#faq:-why-didnt-findclass-find-my-class) from inside our Bevy app, due to how the thread model works. Thus we cannot call `startActivity` directly from our JNI code, since we cannot get a handle to the class object.

So, to get around this, this solution defines a custom `NativeActivity` class which extends the typical `NativeActivity` and adds some useful methods that we _can_ access from the JNI. See `mobile/kotlin/NativeActivity.kt` for more details. These methods can define the appropriate logic for starting our NativeActivity and receiving results from the `AdActivity` that loads and renders the ad. Each of the public methods defined in `NativeActivity.kt`, namely `startAdActivity`, `didEarnReward`, and `didCancel`, are called from a Bevy system, meaning that Bevy ultimately has access to ad redemption information.

This example also issues log statements that are helpful for understanding the app lifecycle. Importantly, the Bevy app is "paused" while the `AdActivity` is active, so we don't need to do anything special to stop systems from running during that time (for the most part).

There are several TODOs and caveats here, including pre-loading ads and configuring some useful loading screens in case the ad still doesn't render immediately. Additionally, [the Android docs](https://developer.android.com/training/basics/intents/result) suggest using separate methods from Activity classes defined in the `AndroidX` package. These classes support a more thorough implementations of the `startActivityForResult` method (which can be buggy in a few ways on its own, see the docs). Since `NativeActivity` does not provide these methods, a proper solution should ensure that `startActivityForResult` is used in a way that mimics these advanced AndroidX APIs.

## Android Environment Setup

- Install [Android Studio](https://developer.android.com/studio) or [sdkmanager](https://developer.android.com/tools/sdkmanager)

  - These tools will be used to install a bunch of useful packages.

    - Using Android Studio, this can be accomplished with the SDK Manager GUI.
    - Using `sdkmanager`, use the `--list` flag to find the appropriate packages and install them with the `--install` flag.
    - The packages needed include:

      - `build-tools`
      - `cmdline-tools`
      - `cmake`
      - some `platform` (e.g. "Android 33")
      - some `ndk` (e.g. "NDK 26.1.10909125")

  - It is now useful to add some environment variables to your system:

    - `ANDROID_HOME` is the root folder of your Android installation.
    - `ANDROID_SDK_ROOT` is the same as `ANDROID_HOME`; it is supposedly deprecated, but excluding it has caused issues for me at times. TODO: test if I can remove this yet.
    - `ANDROID_NDK_ROOT` is `$ANDROID_HOME/ndk/<your-chosen-ndk-version>`. Be sure to update this if you want to change which NDK you build with.

  - A bunch of these binaries must now be made available in your `PATH`, so that the build system can detect all the required binaries.

    - platform tools: `$ANDROID_HOME\platform-tools`
    - command-line tools: `$ANDROID_HOME\cmdline-tools\latest\bin` (or a version string instead of `latest` if relevant)
    - an llvm toolchain: `$ANDROID_NDK_ROOT\toolchains\llvm\prebuilt\<your-os>\bin` (note the platform string on your own environment, e.g. `linux-x86_64`)

- Download [`Gradle`](https://gradle.org/install/#manually), it doesn't matter where you put it

  - Add a `GRADLE_HOME` environment variable to this location
  - Add `$GRADLE_HOME\bin` to your PATH

[NiklasEi's game template docs](https://github.com/NiklasEi/bevy_game_template/blob/main/README.md#deploy-mobile-platforms) is another useful resource.

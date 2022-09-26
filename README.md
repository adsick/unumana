# Unumana (um)
"one handed" in [Esperanto](https://en.wikipedia.org/wiki/Esperanto)

The main purpose of this project is to build a cross platform text editor that will have an advanced input system that is based on state machines. A very basic example of what it will be able to do is binding commands on long keypresses.

## features
* <details><summary>modal - you know vim</summary>try and create your custom new modes like numeric mode where all the numbers live on the home row!</details>
* <details><summary>no keyrepeattttt... (unless makes sence)</summary>keyrepeat is 99% enabled by default everywhere in your OS, but how often you find yourself using it with keys like 'o' ar 'a'? (we're not talking about fellow flooders here). key repeat is disabled by default in unumana, but can be emulated in case needed</details>
* <details><summary>virtual keymaps</summary> unumana maps scancodes ignoring your OS, it means that using it with "wrong" layout (e.g. cyrillic) is not a problem as it stands with terminal editors like vim or helix.</details>
* <details><summary>suitable for minimal keyboards</summary>Keyboards are too big. I don't like Fn keys so by default the're not gonna be used. Numpads? same. Note: you still can use whatever you desire, it's all customizable</details>

### future
* extensive configurability (keymaps, plugins, themes)
* ide features (highlighting, rust-analyzen integration)
I'm not sure if we'll get this far, <span title="but I hope">sed mi esperas...</span>

bevare of these:
* <details><summary>not intended for real word use (as of now)</summary>the project is in early stages of development and therefor is unstable and very feature incomplete, so you should not consider using it in your work or life (you are welcome to try if you are bored though)</details>
* <details><summary>some input lag is very possible</summary> some is principial due to how keybind resolving will work (sometimes you need to wait before you can detect certain action), some is pure technical and hopefully could be improved on in future updates.</details>
* <details><summary>not perfectly smooth (scrolling)</summary> this is probably due to inconsistent time deltas that Bevy provides, see [issue](https://github.com/bevyengine/bevy/issues/4669)</details>


### stack
I've decided to pick <span style="background-color: gray">[Bevy](bevyengine.org/)</span> game engine for the front end because I believe it will facilitate things like window creation and text rendering, but control can be limited if we're not diving in engine's source or start writing custom shaders.

Also there is [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon) for some vector graphics, but it's use is questionable, I hope that Bevy can give us fancy drawing capabilities, but now it is limited to basic meshes and advanced shaders.

## development
contributions are welcome but please create an issue if you want to change something sensible.

generally I try to target latest stable Rust, Bevy and everything else, but there might be compatibility issues between crates that are not up to date with latest Bevy so we might stick to stable versions or fixed git  

## workspace setup
I'm using workspaces to deal with Rust's long compile times and huge 'target's (Bevy has 300+ deps) and compiles in 5 min on my current laptop.

You still will have to compile deps once, but workspaces will allow you to reuse results of the first compilation for other projects.

here is how to do it:

1) clone Bevy: `git clone https://github.com/bevyengine/bevy.git`
2) create a dir for your projects : 
```bash
cd bevy; mkdir adsick #replace with yours
```
3) modify bevy/Cargo.toml to include crates in adsick:
```toml
[workspace]
exclude = ["benches", "crates/bevy_ecs_compile_fail_tests"]
members = [
  "adsick/*", #put yours here
  "crates/*",
  ...
```

4) currently we have [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon) as a dependency so you might want to
```bash
cd crates
git clone https://github.com/bevyengine/bevy.git
```
and modify it's Cargo.toml to target your local Bevy:
```toml
[dependencies]
bevy = {path = "../../../bevy", default-features = false, features = ["bevy_sprite", "bevy_render", "bevy_core_pipeline", "bevy_asset"]}
lyon_tessellation = "0.17"
svgtypes = "0.5"

[dev-dependencies]
bevy = {path = "../../../bevy", default-features = false, features = ["x11", "bevy_asset"]}
```
<details><summary>hmm...</summary>It would be cool if cargo allowed to specify the root of the current workspace instead of that ../../../../</details>

5) now we are finally ready to
```bash
cd adsick
git clone https://github.com/bevyengine/bevy.git
cd unumana
cargo run --release
```

if you see complaints about window server (or similar) try disabling wayland feature in Cargo.toml.

*happy hacking*
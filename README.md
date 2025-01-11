# sky-labs
Just another Game Engine for studying (in Rust)

## Roadmap

Labels:

- 🚀 - Launched
- 🔧 - Needs some fix
- 🛣️ - Working on...
- 🐢 - Might take a while (not even started)

### Windows Support 🛣️
To acquire knowledge of most recent technology on rendering techniques, 
the plan is to setup rendering with DirectX 12, 
even though it surely is not the best option for such a simple project.

- Launch Window 🚀
    - Event based 🚀
    - Loop based 🚀
    - Fullscreen support 🐢
- Input handling 🛣️
    - Keyboard input 🛣️
    - Controller input 🐢
- Support for 2D rendering 🛣️
    - Basic geometries 🛣️
    - Sprites 🐢
    - Textures 🐢
    - Hit testing 🐢
    - Custom shaders 🐢 
    - Sample game 🛣️
- Support for 3D rendering 🐢
    - Still learning what to do here... 🐢

### Linux Support 🐢
For linux, as in Windows, the plan is to setup rendering with Vulkan and port the windowing into EGL (or maybe some other lib).
Will start after having the Windows sample game done.

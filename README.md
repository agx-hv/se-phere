<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/sphere_tsp.png" alt="sphere.io">
</p>

<p align="center">
    <b>Sphere.io - A Multiplayer 3D Game Development Project</b>
</p>

Sphere.io is a multiplayer 3D game developed within the constraints of the OpenGL graphics module. Players control spheres in a dynamically evolving landscape, aiming to reach a central goal while strategically reshaping the terrain to impede opponents' progress. The game features a custom physics engine, realistic sphere interactions, and minimalist aesthetics focused on gameplay mechanics and user interaction.


## Getting Started

### Installation
- Download Rit - <https://git-scm.com/downloads>
- Download Rust - <https://www.rust-lang.org/learn/get-started>
- Download Windows Build Tools - <https://visualstudio.microsoft.com/downloads/>  
 - Check C++ Tools  
- Download CMake - <https://cmake.org/download/>
```bash
git clone https://github.com/agx-hv/se-phere
```


## Running The Game

### Server Config
- Get IP of server host via ipconfig
- Run the RunServer.sh file or use the following code:
```bash
cargo run --bin server --release
```

### Client Config
- Run the RunClient.sh file or use the following code:
```bash
cargo run --bin title --release
```


## Instructions
```W``` key to Roll Forward  
```S``` key to Roll Backward  
```A``` key to Spin Left  
```D``` key to Spin Right  
```Spacebar``` to Jump  
```LeftClick``` to Raise Ground  
```RightClick``` to Lower Ground  
```Esc```  to Quit Application

## Gameplay Loop

### Login to the Main Game via the GUI
<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/gui.gif" alt="gui showcase">
</p>

### Control Your Sphere with WASD and Score Points!
<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/score.gif" alt="do_score( )">
</p>

### Raise the Ground to Block Others with Left Click
<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/build.gif" alt="do_build( )">
</p>

### Lower the Ground to Eliminate Others with Right Click
<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/kill.gif" alt="do_kill( )">
</p>

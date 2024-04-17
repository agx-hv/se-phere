<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/sphere_tsp.png" alt="sphere.io">
</p>

<p align="center">
    <b>Sphere.io - A Multiplayer 3D Game Development Project</b>
</p>

Sphere.io is a multiplayer 3D game developed within the constraints of the OpenGL graphics module. Players control spheres in a dynamically evolving landscape, aiming to reach a central goal while strategically reshaping the terrain to impede opponents' progress. The game features a custom physics engine, realistic sphere interactions, and minimalist aesthetics focused on gameplay mechanics and user interaction.


## Getting Started

### Installation
- Download git - <https://git-scm.com/downloads>
- Download rust - <https://www.rust-lang.org/learn/get-started>
- Download windows build tools - <https://visualstudio.microsoft.com/downloads/>  
 - Check C++ tools  
- Download cmake - <https://cmake.org/download/>
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
```w``` key to Roll Forward  
```s``` key to Roll Backward  
```a``` key to Spin Left  
```d``` key to Spin Right  
```Spacebar``` to Jump  
```LeftClick``` to Raise Ground  
```RightClick``` to Lower Ground  
```esc```  to Quit Application

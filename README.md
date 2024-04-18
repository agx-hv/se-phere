<p align="center">
  <img height="100" src="https://github.com/agx-hv/se-phere/blob/main/assets/readme/sphere_tsp.png" alt="sphere.io">
</p>

<p align="center">
    <b>Sphere.io - A Multiplayer 3D Game Development Project</b>
</p>

Sphere.io is a multiplayer 3D game developed within the constraints of the OpenGL graphics module. Players control spheres in a dynamically evolving landscape, aiming to reach a central goal while strategically reshaping the terrain to impede opponents' progress. The game features a custom physics engine, realistic sphere interactions, and minimalist aesthetics focused on gameplay mechanics and user interaction.

## Quickstart (Playing Sphere.io From Release)

For hosting a server, see below.

### Windows

unzip and run ```title.exe```

### Unix

unzip and run ```title```

### Mac

Build from source, we don't own macs to test, sorry :\)

## Building From Source

### 1a. Installation (Windows)

- Download Git - <https://git-scm.com/downloads>
- Download Rust - <https://www.rust-lang.org/learn/get-started>
- Download Windows Build Tools - <https://visualstudio.microsoft.com/downloads/>  
  - Select C++ Tools  
- Download CMake - <https://cmake.org/download/>  
- Clone the repository

```bash
git clone https://github.com/agx-hv/se-phere
```

### 1b. Installation (Mac/Linux)

Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository

```bash
git clone https://github.com/agx-hv/se-phere
```

### 2. Building binaries

Build all binaries

```bash
cargo build --release
```

### 3. Server Setup

- Get IP of server host (e.g using ```ipconfig```)
- Run the RunServer.sh file or use the following commands:

Build and Run Server

```bash
cargo run --bin server --release
```

### 4. Client Setup

- Run the RunClient.sh file or use the following commands:

Build and Run Client

```bash
cargo run --bin title --release
```

## Game Instructions

### Keyboard Controls

```W``` key to Roll Forward  
```S``` key to Roll Backward  
```A``` key to Spin Left  
```D``` key to Spin Right  
```Spacebar``` to Jump  
```F``` key to Toggle Fullscreen  
```Esc```  to Quit Application

### Mouse Controls

```LeftClick``` to Raise Ground  
```RightClick``` to Lower Ground  
```ScrollForward``` to Zoom Camera In  
```ScrollBackward``` to Zoom Camera Out  
Move Cursor to Screen Edges to Spin and Tilt Camera

## Gameplay Loop

### Login to the Main Game via the GUI

<p align="left">
  <img height="300" src="https://github.com/agx-hv/se-phere/blob/main/assets/readme/gui.gif" alt="gui showcase">
</p>

### Control Your Sphere with WASD and Score Points

<p align="left">
  <img height="300" src="https://github.com/agx-hv/se-phere/blob/main/assets/readme/score.gif" alt="do_score( )">
</p>

### Raise the Ground to Block Others with Left Click

<p align="left">
  <img height="300" src="https://github.com/agx-hv/se-phere/blob/main/assets/readme/build.gif" alt="do_build( )">
</p>

### Lower the Ground to Eliminate Others with Right Click

<p align="left">
  <img height="300" src="https://github.com/agx-hv/se-phere/blob/main/assets/readme/kill.gif" alt="do_kill( )">
</p>

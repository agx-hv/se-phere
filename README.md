# Se-phere

Rust OpenGL sephere game

## How to be a rusty window

0. Download git - <https://git-scm.com/downloads>
1. Download rust - <https://www.rust-lang.org/learn/get-started>
2. Download windows build tools - <https://visualstudio.microsoft.com/downloads/>  
 2.1 check C++ tools  
3. Download cmake - <https://cmake.org/download/>
4. ``` git clone https://github.com/agx-hv/se-phere ```
5. ``` cd .\se-phere\ ```

## Running The Game

1. get IP with ipconfig
2. ``` cargo run --bin server --release ``` or run RunServer.sh
3. set SERVER_IP_ADDR (line 42) in clinet\src\main.rs to ip address, changing '.' to ','
4. ``` cargo run --bin client --release ``` or run RunClient.sh

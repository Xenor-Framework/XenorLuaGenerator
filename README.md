[![BSD 3-Clause](https://img.shields.io/badge/License-BSD3-Claus.svg)](https://opensource.org/license/bsd-3-clause)

# 📖 Xenor Lua Generator
#### Part of the XenorSDK

A static website lua documentation generator which you can customize. Outputs ready-to-use website files. It's that simple! 🚀

## How-to ❓

#### 1. Create/Copy template folder

A template folder must be present in the same location as XenorLuaGenerator/.exe in
```bash
./template/<your_files>
```
*Use the default template folder provided in this repository, simply move to the same folder where the executable XenorLuaGenerator is.*

*A showcase docs.json is included so you can test it out! **Do not use docs.json from this repository, build your own using the executable!***

#### 2. Generate docs.json and build website

Run XenorLuaGenerator with arguments specified below to build your website.
First we need a docs.json which the web builder uses to identify all your functions.
```bash
./XenorLuaGenerator <FULL PATH TO FOLDER CONTAINING LUA FILES AND SUBFOLDERS>
```

***Example***
```bash
./XenorLuaGenerator /home/MyPC/gamemode
```

*This will run a recursive scan on that folder and finds all your .lua files and generates a docs.json based on your comments and generates you your website files located in ./dist/<FINAL WEBSITE FILES> ready to be uploaded to GitHub pages for example!*

*It can be something like <PATH TO GMOD>/gamemodes/my_gamemode*

## Compiling 🛠

Run this command inside the project folder to build your own binary
```bash
cargo build --release
```

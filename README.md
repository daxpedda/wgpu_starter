# **WGPU starter with winit 0.30.5 and WGPU version 23.0.1**

_Date: 14/12/2024_

## **What is it?**

This is an example as an introduction to **WGPU**. In this example we simply draw the famous triangle on a window.
This is based on the example from Learn **WGPU** (https://sotrh.github.io/learn-wgpu/) but refactored to update to the latest lib/API version.
I also took inspiration from the Youtube series from GetIntoGameDev  (https://www.youtube.com/watch?v=I8Uf7akOYo0).

## Why?

Winit 0.30.5 has introduced breaking changes to its API. The examples on Learn GPU and GetIntoGameDev, no longer work with latest version of winit and WGPU.
I had troubles finding online documentation for **Winit** updated API.

Luckily I found this brilliant code that resolves the issue:
https://github.com/w4ngzhen/wgpu_winit_example
So credit to them!
The refactoring around the window is neat using ARC to be able to get a reference on a mutable reference asynchronously.

![Rainbow_triangle](https://github.com/user-attachments/assets/29c75acc-308e-4324-9bdb-38dea9418bb8)

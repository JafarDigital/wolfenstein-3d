# 🕹️ Wolfenstein 3D Raycasting in Rust

This is an experimental project where I explored **raycasting** and how it can be implemented using **Rust**. Based on *Wolfenstein 3D*, this project focuses on the rendering logic that gives the illusion of 3D environments on a 2D grid.

## Purpose

The primary goal was to:

- Understand how **raycasting** works at the algorithmic level.
- Learn how to implement it using **Rust**.
- Explore rendering technique without relying on modern 3D engines.

## What is Raycasting?

Raycasting is a rendering technique used in early 3D games to project a 2D map into a pseudo-3D perspective by **casting rays** from the player's point of view. It doesn’t simulate full 3D geometry but gives a convincing 3D illusion by calculating the distance between the player and walls in a grid.

### How it works theoretically:

1. **Field of View**: The screen is divided into vertical slices (columns). Each column corresponds to a ray.
2. **Ray Direction**: For each ray, we calculate its angle based on the player's current orientation and field of view.
3. **Grid Traversal**: Using **DDA (Digital Differential Analyzer)** or a similar algorithm, we determine where each ray intersects with walls on the 2D map.
4. **Distance Calculation**: The distance from the player to the wall hit by the ray is computed.
5. **Projection**: Using that distance, we draw a vertical line whose height is inversely proportional to the distance — creating the illusion of depth.
6. **Rendering**: Each vertical line is drawn on the screen as part of the 3D view.

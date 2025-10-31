# many_body_problem_bevy

After running into complexity while building my real-time GPU fluid simulation in Bevy (`bevy_gpu_fluid`), I realized I needed a simpler, more focused playground to truly understand shaders **especially `@compute` and `@vertex` stages and frame update timing** 😭.

This project is my attempt to explore **GPU shaders** in a simpler physics system: the **many-body gravitational problem**.

Instead of jumping back into full SPH and pressure kernels, I’ll first model:
- 2-body gravitational systems
- 3-body chaotic orbits
- N-body problem

On the way I will test the shaders!

The goal is to learn:
- How to write and dispatch shaders
- How to bind and synchronize GPU buffers in Bevy
- How to reason about simulation time vs frame time

Before I return to the fluid sim and scream why the `@vertex` shaders don't work correctly.
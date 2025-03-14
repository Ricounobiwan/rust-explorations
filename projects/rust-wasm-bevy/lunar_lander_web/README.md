# 🚀 Lunar Lander in Rust & Bevy (WebAssembly)

## Plan

1. Project Setup (Rust + Bevy + WASM)
2. Render a simple window
3. Add a basic lander sprite
4. Implement gravity
5. Add controls (left, right, thrust)
6. Add terrain & landing mechanics
7. Polish with UI & sound
8. Deploy the WebAssembly app

## Step 1: Project Setup

We’ll set up a Rust Bevy project targeting WebAssembly (WASM).

### 1.1 Install Dependencies

Before we start, ensure you have the required tools installed:

```bash
    // Rust (latest stable)
    rustup update
    // WebAssembly target support
    rustup target add wasm32-unknown-unknown
    // Trunk (for building & serving WASM apps)
    cargo install trunk
    // Trunk allows us to easily compile and serve our WebAssembly application.
```

### 1.2 Create a New Rust Project

Run the following commands to initialize the project:

```bash
  cargo new lunar_lander_web
  cd lunar_lander_web
```

### 1.3 Add Bevy and WebAssembly Support

Edit Cargo.toml and add Bevy as a dependency:

```bash
  [dependencies]
  bevy = "0.13"
```

Bevy supports WebAssembly out of the box, so no additional dependencies are required.

### 1.4 Create an HTML File for WebAssembly

Create a new file named index.html in the project root:

```html
<!DOCTYPE html>
  <html lang="en">
  <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Lunar Lander</title>
  </head>
  <body>
      <script type="module">
          import init from "./lunar_lander_web.js";
          init();
      </script>
  </body>
  </html>
```

### 1.5 Create a Minimal Bevy App

Now, edit src/main.rs and set up a simple Bevy application:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Load default Bevy plugins (window, rendering, input, etc.)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());
}
```

This initializes Bevy with a 2D camera, preparing the scene for rendering.

### 1.6 Build and Run

Now, let's test the setup by compiling and running it in the browser.
Run in the Browser

```bash
trunk serve --open
```

This will:

- Compile our Bevy app to WebAssembly
- Start a local server at <http://127.0.0.1:8080/>
- Open the game in your browser

#### If error related to: UUID

- 1️⃣ Add the Correct uuid Features for WASM

Edit your Cargo.toml file and add:

```bash
[dependencies]
bevy = { version = "0.13", features = ["dynamic_linking"] }
uuid = { version = "1.6", features = ["js"] }
````

This does two things:

- Adds the js feature to uuid, allowing it to use JavaScript’s built-in random number generator (crypto.getRandomValues()).
- Enables dynamic_linking for Bevy, which improves WASM compatibility.

- 2️⃣ Clean and Rebuild the Project

Now, clean the project and try rebuilding:

```
  cargo clean
     Removed 2264 files, 1.3GiB total
  trunk serve --open
```

This should solve the issue and allow the game to run in the browser.

- 🚀 Why Does This Work?

- The js feature tells uuid to use JavaScript’s randomness.
- Bevy works better with WebAssembly when dynamic_linking is enabled.
- Clearing old builds ensures the fixed settings are applied.

### 1.7 First Commit

Now, let's save our progress in Git:

```bash
cd rust-explorations/projects/rust-wasm-bevy
git add lunar_lander_web
git commit -m "Setup Rust + Bevy + WASM project"
```

- Next Steps

Now that the basic setup is complete, the next steps are:

- Rendering a background (moon surface)
- Adding the Lunar Lander sprite
- Implementing physics (gravity, thrust)
- Adding keyboard controls

## 🚀 Step 2: Rendering a Background (Moon Surface)

Now that our WebAssembly setup is working, let's make sure something actually renders in the browser!

We will:

- Set a background color (black for space)
- Render a simple rectangle to represent the moon's surface
- Verify that everything is running correctly in the browser

### 🛠️ 2.1 Modify src/main.rs to Add a Background

Update your main.rs to include:

- A black background (for space)
- A gray rectangle (for the moon’s surface)

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Black background
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera (needed to see objects)
    commands.spawn(Camera2dBundle::default());

    // Spawn the moon surface as a gray rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.3), // Gray color for the moon
            custom_size: Some(Vec2::new(600.0, 50.0)), // Wide rectangle
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -250.0, 0.0), // Position at the bottom
        ..Default::default()
    });
}
```

### 🚀 2.2 Run and Test in the Browser

Rebuild and serve your WebAssembly app:

`trunk serve --open`

You should now see:
    - A black background (space)
    - A gray rectangle at the bottom (moon surface)

If you still see a blank page:
    - Check the browser console (F12 → Console tab) for errors.
    - Ensure trunk is running correctly.
    - Force reload (Ctrl + Shift + R or Cmd + Shift + R on Mac).

### ✅ Step 2: Commit Your Fixes

```bash
git add src/main.rs
git commit -m "Updated to Bevy 0.13 API and fixed trunk server issue"
```

### 📌 Next Step: Add the Lunar Lander Sprite

Now that rendering works, we’ll:

- Add a lander sprite
- Apply gravity
- Add keyboard controls

## 🚀 Step 3: Adding the Lunar Lander Sprite

Now that we have the black space background and gray moon surface, it's time to add the Lunar Lander itself!

- 🎯 Goals for This Step

  - Add a simple lander sprite
  - Position it above the surface
  - Ensure it's visible in the browser
  - Commit the changes

### 1️. Modify setup() to Build a Lander shape

Create a group of shapes (octagon + legs).
Update main.rs

```rust
use bevy::prelude::*;
use bevy::math::primitives::{RegularPolygon, Rectangle};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, ColorMaterial};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Black background
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, // ✅ ColorMaterial for 2D
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn the moon surface as a gray rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(600.0, 50.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        ..Default::default()
    });

    // 🛸 Lander Parts
    let lander_color = materials.add(ColorMaterial::from(Color::WHITE));

    // Octagonal main body
    let octagon = Mesh::from(RegularPolygon::new(15.0, 8));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(octagon)),
            material: lander_color.clone(),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..Default::default()
        },
    ));

    // Landing legs
    let leg = Mesh::from(Rectangle::new(10.0, 2.0)); // ✅ Fix: Correct API usage
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(leg.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(-10.0, 85.0, 0.0).with_rotation(Quat::from_rotation_z(0.4)),
            ..Default::default()
        },
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(leg.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(10.0, 85.0, 0.0).with_rotation(Quat::from_rotation_z(-0.4)),
            ..Default::default()
        },
    ));

    // Lower platform
    let platform = Mesh::from(Rectangle::new(20.0, 2.0)); // ✅ Fix: Correct API usage
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(platform.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(0.0, 80.0, 0.0),
            ..Default::default()
        },
    ));
}
```

Sometimes, if errors occur, necessary to:

```bash
  cargo clean
  trunk clean
  trunk serve --open
```

#### The Faster Workflow

Instead of cleaning every time, try this:

First, just restart Trunk

```bash
  kill -9 $(lsof -t -i:8080)  # Kill the process if needed
  trunk serve --open
```

If errors persist, then try cleaning:

```
  cargo clean && trunk clean
  trunk serve --open
```

### Let's refactor before moving forward

#### Step 1: Create a components Folder

Inside your src/ folder, create a components/ directory:

mkdir src/components

#### Step 2: Split the Code into Modules

We will create three separate files:

    lander.rs → Handles the lander entity (shape, color, position).
    environment.rs → Handles the moon surface.
    setup.rs → Handles game setup (spawning all objects).

- 📌 src/components/lander.rs (Lander Module)

```rust
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, ColorMaterial};
use bevy::math::primitives::{RegularPolygon, Rectangle};

pub fn spawn_lander(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let lander_color = materials.add(ColorMaterial::from(Color::WHITE));

    // Octagonal main body
    let octagon = Mesh::from(RegularPolygon::new(15.0, 8));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(octagon)),
            material: lander_color.clone(),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..Default::default()
        },
    ));

    // Landing legs
    let leg = Mesh::from(Rectangle::new(10.0, 2.0));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(leg.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(-10.0, 85.0, 0.0).with_rotation(Quat::from_rotation_z(0.4)),
            ..Default::default()
        },
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(leg.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(10.0, 85.0, 0.0).with_rotation(Quat::from_rotation_z(-0.4)),
            ..Default::default()
        },
    ));

    // Lower platform
    let platform = Mesh::from(Rectangle::new(20.0, 2.0));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(platform.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(0.0, 80.0, 0.0),
            ..Default::default()
        },
    ));
}
```

- 📌 src/components/environment.rs (Moon Surface Module)

```rust
use bevy::prelude::*;

pub fn spawn_environment(mut commands: Commands) {
    // Moon surface
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(600.0, 50.0)), // Wide rectangle
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        ..Default::default()
    });
}
```

- 📌 src/setup.rs (Setup Module)

```rust
use bevy::prelude::*;
use crate::components::lander::spawn_lander;
use crate::components::environment::spawn_environment;

pub fn setup_game(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_environment(commands);
    spawn_lander(commands, meshes, materials);
}
```

#### Step 3: Modify main.rs to Use These Modules

Now, clean up src/main.rs to just initialize Bevy and call our setup function.

```rust
mod components;
mod setup;

use bevy::prelude::*;
use setup::setup_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_game)
        .run();
}
```

###

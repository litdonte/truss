# Truss

A framework-agnostic Rust library that brings compile-time verification to HTMX routes, swap targets, and component data.

Catch mismatched routes and invalid swap targets at compile time — not in the browser.

> This library is in early development. See [DESIGN.md](DESIGN.md) for the full design document.

## The Problem

HTMX is a powerful library for building dynamic UIs without the overhead of a frontend framework. But it introduces failure modes that are easy to miss:

- **Routes are plain strings** — a typo produces a silent 404 with no error, no crash, no log
- **Swap targets are plain CSS selectors** — a missing element fails silently in the same way
- **Errors are runtime-only** — debugging means hunting through the developer console for string clues

Some of these problems have been addressed by existing crates like `axum-routing-htmx` and `htmxology`, but they are tightly coupled to Axum and do not solve swap target verification or provide a component model where components own their target ids.

Truss fills that gap.

## Goals

- **Compile-time route verification** — invalid routes are compiler errors, not silent 404s
- **Compile-time swap target verification** — missing targets are caught before the app runs
- **Framework agnostic** — works with Axum, Actix, or any Rust backend
- **Component-driven architecture** — components own their data, render logic, and target ids
- **Element macros** — ergonomic HTML generation with type-safe HTMX attributes

## How It Works

Every key element of an HTMX template is a Rust type. Routes are enums. Swap targets are components, not CSS strings. If a target doesn't exist, the compiler tells you.

```rust
type SceneId = Id<ScenePanel>;

// Define your routes — single source of truth
#[routes]
pub enum Route {
    #[get("/scenes")]
    Scenes,
    #[get("/scene/{id}")]
    Scene(SceneId),
    #[post("/scene")]
    CreateScene,
}

// Define query parameters for the Scene route
#[query]
pub enum SceneQuery {
    Character(String),
    Act(usize),
}

// Define a child component
#[component]
pub struct SceneOverview {
    scene: Scene,
}

impl SceneOverview {
    fn render(&self) -> Html {
        fragment! {
            div! {
                h1! { self.scene.title }
                p! { self.scene.description }
            }
        }
    }
}

// Define a parent component
#[component]
pub struct ScenePanel {
    id: SceneId,
    overview: SceneOverview,
}

impl ScenePanel {
    fn render(&self) -> Html {
        let params = &[SceneQuery::Character("Aerymis".into())];

        fragment! {
            div! {
                button! {
                    HxGet(Route::Scene(self.id).query(params))
                    HxTarget(self.overview)  // compile-time verified — not a CSS string
                    "Load Scene"
                }

                self.overview
            }
        }
    }
}

// Handler references the Route enum variant directly
// Route string is pulled from Route::Scene — no drift possible
#[handler(Route::Scene)]
async fn get_scene(id: SceneId, db: Db) -> ScenePanel {
    let scene = db.get_scene(id).await?;
    ScenePanel {
        id,
        overview: SceneOverview { scene },
    }
}
```

`HxTarget(self.overview)` takes a component, not a string. If the target doesn't exist or the type is wrong, it doesn't compile.

## Status

| Phase | Description           | Status         |
| ----- | --------------------- | -------------- |
| 1     | Core types and traits | ✅ Complete    |
| 2     | Derive macros         | ✅ Complete    |
| 3     | Route macro           | ✅ Complete    |
| 4     | Element macros        | ✅ Complete    |
| 5     | Query parameters      | 🔄 In progress |
| 6     | Handler macro         | 📋 Planned     |
| 7     | Polish and release    | 📋 Planned     |

## Workspace

- `truss` — core types, traits, and HTML generation
- `truss-macros` — procedural macros

## License

MIT

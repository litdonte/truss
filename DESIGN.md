# Truss Design Doc

## Problem Statement

HTMX is a library that extends the functionality of HTML and simplifies the technical overhead of creating dynamic, modern UIs in modern browsers. It gives developers access to AJAX, CSS Transitions, WebSockets and Server Sent Events directly in HTML using attributes. But nothing is perfect, and HTMX introduces a few challenges. In particular:

1. Routes are plain strings, which means a typo would produce a silent 404. It doesn't show the user an error, it doesn't crash, it doesn't log anything obvious.
2. Swap targets are plain string CSS selectors, which means non-existing elements fail silently, similarly to route typos.
3. Errors are only known at runtime, specifically as strings in the developer console. Debugging is not developer friendly. When actions don't happen as expected, developers aren't given a reason why.

Some of these problems have been solved by existing tools. In the Rust ecosystem, `htmxology` and `axum-routing-htmx` aim to solve the route safety, but they're closely coupled with the Axum framework. They don't solve the swap target verification, error analysis, and debugging problems. The ecosystem could benefit from a crate that is framework agnostic and verifies routes, targets, and data at compile time.

Truss aims to fill that gap — a framework agnostic Rust library that brings compile time verification to HTMX routes, swap targets, and component data.

## Goals

- **Compile time route verification** — invalid routes are compiler errors, not silent 404s
- **Compile time swap target verification** — missing targets are caught before the app runs
- **Framework agnostic** — works with Axum, Actix, or any Rust backend
- **Component driven architecture** — components own their data, render logic, and target ids
- **Element macros** — ergonomic HTML generation with type safe HTMX attributes

## Non-Goals

- Truss is not a replacement for HTMX — it's a templating engine that generates type safe HTMX markup
- Not a full frontend framework — no client side state management
- Not responsible for database safety, authentication, or security in transit

## Prior Art

Several crates in the Rust ecosystem have attempted to improve the HTMX development experience:

**`axum-htmx`** provides extractors and responders for HTMX headers within Axum. It makes working with HTMX request and response headers ergonomic, but does not address route type safety or swap target verification.

**`axum-routing-htmx`** provides type-safe routes in Axum tailored for HTMX. Path and query parameters are type-checked at compile time, which solves the silent 404 problem for route parameters. However it is tightly coupled to Axum and does not address swap target verification or the component model.

**`htmxology`** is the most complete existing solution — a full stack web framework for Rust that brings together HTMX and Axum. Routes are defined as Rust enums, eliminating typos and broken links. It provides compile time guarantees and first class HTMX support. However like `axum-routing-htmx`, it is tightly coupled to Axum and does not address swap target verification or provide a component driven architecture where components own their target ids.

Truss builds on the ideas established by these crates while addressing their limitations, such as framework agnosticism, swap target verification, and a component model that makes invalid states structurally impossible.

## Core Concepts

Truss is built around type driven development, such that, every key element of creating an HTMX template is a Rust type. At a high level:

- **Component** - the central building block. It owns the data, the target identification, and render logic. Aside from the HTMX template the developer creates, the `#[component]` derive macro handles the rest.
- **Route** — a typed enum with variants that generate HTMX attributes. Route definitions live exclusively in the `Route` enum and handlers reference variants directly via `#[handler]`, preventing drift.
- **Query Parameters** - a typed enum via the `#[query]` derive macro with variants of valid parameters for a given route.
- **Element Macros** - `fragment!`, `html!`, `div!`, `button!`, etc. generate HTMX annotated HTML. `fragment!` produces partial HTML responses for HTMX swaps. `html!` produces full page responses with the HTMX script embedded automatically.
- **Swap Target Verification** - targets are referenced by components, not CSS selector strings. This prevents silent errors and missing targets trigger a compiler error.
- **Component Trait** - the trait `#[component]` derives automatically. It defines `id()`, `with_id()`, and `render()`. Being a trait means any component can be passed as a swap target, enabling compile time target verification.
- **IntoQueryParam Trait** - this is the trait `#[query]` derives. It converts enum variants into key/value pairs.
- **`Id<T>`** — a typed identifier that carries the type of the entity it identifies through a `PhantomData` marker. Prevents passing identifiers of the wrong type, an `Id<ScenePanel>` can never be used where an `Id<SceneOverview>` is expected.

## API Design

The following example demonstrates the full Truss API. A `Route` enum defines
all routes in one place. Components are defined with `#[component]` and implement
`render()` to produce HTMX annotated HTML. Handlers reference route enum variants
directly via `#[handler]`, pulling the route string from the enum definition and
eliminating drift. Query parameters are typed through `#[query]` enums.

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
                    HxTarget(self.overview)
                    "Load Scene"
                }

                self.overview
            }
        }
    }
}

// Full page response — HTMX script embedded automatically
#[handler(Route::Home)]
async fn home() -> Page {
    html! {
        head! { title! { "My App" } }
        body! {
            div! { "Welcome" }
        }
    }
}

// Partial response — HTMX fragment
#[handler(Route::Scene)]
async fn get_scene(id: SceneId, db: Db) -> Html {
    let scene = db.get_scene(id).await?;
    let overview = SceneOverview::new(scene);
    ScenePanel::new(id, overview).render()
}
```

## Implementation Plan

**Phase 1 — Core Types and Traits** ✅
Start with the foundation that everything else builds on. No macros yet, just the types and traits:

- Component trait — id(), with_id(), render()
- IntoQueryParam trait
- Id<T> struct with PhantomData
- Html type — what render() returns
- Route trait — what all route enums implement

**Phase 2 — Derive Macros** ✅
Now automate what you proved works manually in Phase 1:

- `#[component]` — derives Component trait
- `#[query]` — derives IntoQueryParam trait
- `#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[patch]` — route attributes on enum variants

**Phase 3 — Route Macro** ✅

- `#[routes]` — generates `impl RouteInfo` for route enums, preventing drift between route definitions and handler registrations

**Phase 4 — Element Macros**

- Core: `fragment!`, `html!`, `div!`, `button!`, `a!`, `form!`, `input!`
- `fragment!` — partial HTMX response wrapper, returns `Html`
- `html!` — full page wrapper, returns `Page` with HTMX script embedded automatically
- Via `element_impl!` delegation pattern
- Element macros build nodes in an internal Html tree
- Traverses the tree depth first to produce the final HTML string

**Phase 5 — Query Parameters**

- `.query()` method on routes
- `Query<T>` type

**Phase 6 — Handler Macro**

- `#[handler(Route::Scene)]` — reads route string from enum variant, registers handler
- Framework adapters — Axum, Actix

**Phase 7 — Polish**

- Remaining element macros
- Documentation
- Examples
- Benchmarks
- crates.io

## Open Questions

1. How does compile time route registration work exactly — the #[handler] macro needs to find the Route enum at compile time. The mechanism for this isn't fully designed yet.

2. How does element_impl! handle nesting — macros inside macros need careful design to avoid conflicts.

3. Framework agnosticism — how does the handler integrate with different frameworks? Does Truss provide adapters or does the developer wire it up themselves?

4. How are HTMX response headers handled — things like HX-Redirect, HX-Trigger, HX-Refresh. Are these part of Truss or left to the developer?

5. What's the story for non-HTMX requests — if a user navigates directly to /scene/42 without HTMX, the handler still needs to return a full page, not just a fragment.

6. How does `html!` determine which version of HTMX to embed — hardcoded CDN link, configurable version, or vendored?

# Systems and Queries

In Bevy, **systems** are the beating heart of your game: containing all of the necessary logic to actually *make stuff happen*.
Systems are ordinary (if constrained) Rust functions that you use by: 

1. Defining the function with the appropriate argument types.
2. Added to your `AppBuilder` with functions like `.add_system`.
3. Automatically run at the specified times and supplied with data to read and write by Bevy's [scheduler](timing/scheduling-stages.md).

Ordinary systems can have one of four types of arguments:
1. **Queries** (`Query`), which grab the components for all entities which have *all* of the specified components and pass the **query filters**.
2. **Resources** (`Res`, `ResMut` and `Local`), which are global singletons for storing data that isn't associated with a particular entity.
3. **Commands** (`Commands`), for queueing up broad-reaching tasks until the end of the stage.
4. **System-chained arguments** (`In`), which automatically fetch the output of another system with the appropriate task. These are less common, and are discussed in [Chaining Systems](communication/chaining.md) instead.

Thread-local systems (discussed below) have more complete (but not parallelizable) access to our app's state. They accept `World` (which collects all of the entity + component data) and `Resources` arguments instead.

For simple projects, the most important distinction is between **startup systems** and ordinary systems. Startup systems run exactly once, before any ordinary systems run, while ordinary systems will run every tick.
We can add systems to our apps with the [`add_system`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_system) or `add_startup_system` methods:

```rust```

Once you begin to worry about more complex issues of timing, you can use [`add_system_to_stage`] to control which [stage](timing/scheduling-stages.md) it is in.

## Queries and Query Filters

In order to access our components in our systems, we need to supply our system with query arguments.
Queries have [two type arguments](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Query.html) a `WorldQuery` and an optional `QueryFilter`.

`WorldQuery` contains a set of components, and returns those components for all entities that have *all* of those components. 
You can pass it in as either a singleton component or as a tuple:

```rust```

So the `WorldQuery` type argument will do a union on all entities that have the components specified in its tuple and return those components. Then, the `QueryFilter` will restrict that list of supplied entities 

There are several filters that are built into Bevy:
- `With<T>`: Only include entities that have the component `T`. This can be particularly handy when working with marker components, as it lets you extract only the entities with that marker component without grabbing the useless unit struct itself.
- `Without<T>`: Exclude all entities with the component `T`.
- `Added<T>`: Only include entities whose component `T` could have been added during this tick. This picks up entities that are spawned as well.
- `Mutated<T>`: Only include entities whose component `T` *could have* been modified during this tick. Note that you could change a different component on that entity without causing it to be marked as mutated. 
  - [Deep within the engine](https://github.com/bevyengine/bevy/blob/457a8bd17d5f5d30a5a2fb6eabce7fc0b95bfc94/crates/bevy_ecs/src/core/borrow.rs#L168), this is flagged when a mutable reference to our component is dereferenced. 
  If you carefully avoid doing so unnecessarily, you can prevent your component from being marked as mutated unless you actually change its value.
- `Changed<T>`:Only include entities that meet the criteria for either `Added<T>` or `Mutated<T>` during this tick. This is usually what you want, rather than `Added` or `Mutated`.

Be careful when using `Added`, `Mutated` or `Changed`: [right now](https://github.com/bevyengine/bevy/issues/68#issuecomment-751311732), they only detect changes made by systems that ran earlier in the tick; you'll want to put them in later stages. 

Here's an example of how you might use a few different filters. Like with `WorldQuery`, you can combine these types to create more complex filters:

```rust```

## Working with Query Objects

Once you have your query, you'll most commonly want to interact with it through iterables:

```rust```

If you're looking to optimize your code, it may be worth parallelizing the operations you're performing on your queries in particularly heavy systems:

```rust```

You can fetch components from particular entities using the [`query.get`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Query.html#method.get) family of methods:
```rust```

One particularly useful but non-obvious pattern is to work with relationships between entities by storing an `Entity` on one component, then. Here's an example of how it might work. Be mindful though: the `Entity` stored in your component can easily end up stale as entities are removed, and you need to be careful that this doesn't cause panics or logic errors. 

```rust ```

### Thread-Local Systems

When you need to work with [thread-local resources](resources.md) or need complete access to all resources and components (like when saving or loading a game), you can use a [thread-local](https://docs.rs/bevy/0.4.0/bevy/ecs/prelude/trait.System.html#tymethod.run_thread_local) system.

While thread-local systems block all other systems, they give you full mutable access to every component and resource:
```rust ```

Thread-local systems are less performant and harder to reason about than ordinary systems: don't use them unless you have to. 
If you just want to ensure that your systems run one-by-one in a fixed order, use [`SystemStage::serial()`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.SystemStage.html#method.serial) instead.

### Concurrent Systems

For some systems, we may be able to pause the work that we're doing, deferring it to a later tick when we have more time or computing resources available.
By using a [system-local resource](resources.md), we can keep track of our progress, and break out of the system once we're out of time, rather than stalling the entire game.

```rust 
{{#include resources_code/examples/resource_smart_pointers.rs}}
```
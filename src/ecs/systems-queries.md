# Systems and Queries

In Bevy, **systems** are the beating heart of your game: containing all of the necessary logic to actually *make stuff happen*.
Systems are ordinary (if constrained) Rust functions that you use by: 

1. Defining the function with the appropriate argument types.
2. Adding to your `AppBuilder` with functions like `.add_system`.

Systems:
1. Live within a ['Stage'](timing/scheduling-stages.md), which control the broad timing and scheduling strategy of the system.
2. Automatically run and supplied with data to read and write by Bevy's [scheduler](timing/scheduling-stages.md).

Ordinary systems can accept any arguments that implement the [`SystemParam`](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.SystemParam.html) trait. The typical built-in options for this are:
1. **Queries** (`Query`), which grab the components for all entities which have *all* of the specified components and pass the **query filters**.
2. **Resources** (`Res`, `ResMut` and `Local`), which are global singletons for storing data that isn't associated with a particular entity.
3. **Commands** (`Commands`), for queueing up broad-reaching tasks until the end of the stage. See [Commands](communication/commands.md) for an explanation of how these work.
4. **System-chained arguments** (`In`), which automatically fetch the output of the system that is configured to chain into them. These are less common, and are discussed in [Chaining Systems](communication/chaining.md) instead.

Thread-local systems (discussed below) have complete (but not parallelizable) access to our app's state. They accept `World` (which collects all of the entity + component data) and `Resources` arguments instead.

For simple projects, the most important distinction is between **startup systems** and ordinary systems. Startup systems run exactly once, before any ordinary systems run, while ordinary systems will run every tick.
We can add systems to our apps with the [`add_system`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_system) or [`add_startup_system`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_startup_system) methods.

In the following example, we take a look at the skeleton of a reasonably complete small game, to demonstrate how systems would actually be used:

```rust
{{#include systems-queries_code/examples/adding_systems.rs}}
```

## Queries

In order to access our components in our systems, we need to supply our system with query arguments.
Queries have [two type arguments](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Query.html) a `WorldQuery` and an optional `QueryFilter`.

You can create a [`WorldQuery`](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.WorldQuery.html) by providing either a single component or a tuple of components as the first type argument in a `Query` parameter of your system.
That `Query` will return the specified components for all entities that have *all* of those components.

```rust
{{#include systems-queries_code/examples/queries.rs}}
```

### Accessing Heterogenous Data with Queries for Option<C> Components

With the help of [`Option<C>`](https://doc.rust-lang.org/rust-by-example/std/option.html), we can construct more sophisticated queries by understanding the algorithm used to determine which entities are accessed:

1. Begin with a list of all entities.
2. For each type in our `WorldQuery` that is not an `Option`, remove all entities that lack that component.
3. For each of the entities in the list, return the component data for each type in our `WorldQuery`. If it is an `Option`, return the component data for the wrapped type instead, wrapped in an `Option` in case it doesn't exist.

Under the hood, this is done using [archetypes](internals/archetypes.md) rather than entities for performance.

Readers who have experience with relational databases will notice that this results in:
1. An inner join between all ordinary component types in our `WorldQuery`: only providing the data if every field exists.
2. An outer join with all `Option` component types in our `WorldQuery`: providing the data even if the field doesn't exist.

There are a few useful patterns with this functionality:
* Reducing code duplication by handling simple edge cases within the system itself.
* Overriding a default behavior when specialized data exists.
* Determining which variant we received when using `Or` query filters.
   
```rust
{{#include systems-queries_code/examples/option_queries.rs}}
```

### Query Sets

Your systems can have multiple `Query` parameters, but you cannot access the same data in them due to restrictions imposed by Rust's borrow checker.
If you need to, you can bypass this limitation using the `QuerySet` type:

```rust
{{#include systems-queries_code/examples/query_set.rs}}
```

## Query Filters

Once we have the initial list of entities from `WorldQuery`, we can further restrict it using the second optional type parameter of `Query`: [`QueryFilter`](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.QueryFilter.html).

Bevy comes with several filters:
- `With<T>`: Only include entities that have the component `T`. This can be particularly handy when working with marker components, as it lets you extract only the entities with that marker component without grabbing the useless unit struct itself.
- `Without<T>`: Exclude all entities with the component `T`.
- `Added<T>`: Only include entities whose component `T` could have been added during this tick. This picks up entities that are spawned as well.
- `Mutated<T>`: Only include entities whose component `T` *could have* been modified during this tick. Note that you could change a different component on that entity without causing it to be marked as mutated. 
  - [Deep within the engine](https://github.com/bevyengine/bevy/blob/457a8bd17d5f5d30a5a2fb6eabce7fc0b95bfc94/crates/bevy_ecs/src/core/borrow.rs#L168), this is flagged when a mutable reference to our component is dereferenced. 
  If you carefully avoid doing so unnecessarily, you can prevent your component from being marked as mutated unless you actually change its value.
- `Changed<T>`:Only include entities that meet the criteria for either `Added<T>` or `Mutated<T>` during this tick. This is usually what you want, rather than `Added` or `Mutated`.
- `Removed<T>`: Only include entities that have also had the specified component removed during this tick. This is commonly used with a `Query<(Entity, U), Removed<T>>`, to extract the `Entity` identifier and use it to update a second component `U` that stored a relation to another entity.
- `Or<T>:` Combine query filters via a logical OR, rather than the usual AND logic.

Be careful when using `Added`, `Mutated`, `Changed` or `Removed`: [right now](https://github.com/bevyengine/bevy/issues/68#issuecomment-751311732), they only detect changes made by systems that ran before them in the same tick.

Like with `WorldQuery`, you can combine these types to create more complex filters. Here's an example demonstrating their capabilities:

```rust
{{#include systems-queries_code/examples/query_filters.rs}}
```

### Working with Query Objects

Once you have your query, you'll most commonly want to interact with it through iterables.
These implement the [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html) trait via ['QueryIter'](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.QueryIter.html), so you have access to all sorts of convenient functional programming tools:

```rust
{{#include systems-queries_code/examples/query_iter.rs}}
```

### Accessing Specific Entities

One particularly useful but non-obvious pattern is to work with relationships between entities by storing an `Entity` on one component, then. Here's an example of how it might work. Be mindful though: the `Entity` stored in your component can easily end up stale as entities are removed, and you need to be careful that this doesn't cause panics or logic errors.
You can fetch components from particular entities using the [`query.get`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Query.html#method.get) family of methods:

```rust
{{#include systems-queries_code/examples/query_get.rs}}
```

The `Parent` and `Child` components in Bevy, used for defining organizational hierarchies to control positioning, uses this pattern.

## Generic Systems

When working with multiple similar objects, we can use Rust's [generics](https://doc.rust-lang.org/book/ch10-00-generics.html) to allow us to comfortably specialize behavior of our systems.
This pattern allows us to keep our components small (allowing each variant system to run in parallel) and specialize behavior when it matters, without duplicating shared code.

```rust
{{#include systems-queries_code/examples/query_get.rs}}
```

## Thread-Local Systems

When you need to work with [thread-local resources](resources.md) or need complete access to all resources and components (like when saving or loading a game), you can use a [thread-local](https://docs.rs/bevy/0.4.0/bevy/ecs/prelude/trait.System.html#tymethod.run_thread_local) system.

While thread-local systems block all other systems, they give you full mutable access to every component and resource:
```rust
{{#include systems-queries_code/examples/exclusive_systems.rs}}
```
Thread-local systems are less performant and harder to reason about than ordinary systems: don't use them unless you have to. 
If you just want to ensure that your systems run one-by-one in a fixed order, use [`SystemStage::serial()`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.SystemStage.html#method.serial) instead.
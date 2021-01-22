# Commands

[`Commands`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html) are an accumulating list of things that should be done to the game state: either the `World`, which contains our entity-component data, or our `Resources`.

We can access `Commands` in our systems by adding a parameter with the `&mut Commands` type.
This gives us shared access to `Commands`, allowing us to safely add new commands to our list before they are applied at the end of the stage (or immediately before a thread-local system).

That last bit is important: **commands are not applied immediately**.
This gives us the ability to collect them, and apply them efficiently to the game state despite making high-impact changes that touch a great deal of data (like spawning or despawning entities, or changing [archetypes](../internals/archetypes.md)).

This commonly catches new users off guard (as the system they've added didn't seem to do anything). 
It also means that commands are not the right tool when you have a long, complex chains of systems that need to respond quickly to each other.
Other methods of [intersystem communication](./communication/_index.md) are a better fit for these problems.

If you really need to refresh `Commands` immediately, you can create a thread-local system at the desired point in your schedule. Under the hood, this automatically causes the [`apply`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.apply) method to be run.

## Using Commands

The most common uses of `Commands` are:
1. Spawning entities with [`spawn`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.spawn) or [`spawn_batch`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.spawn_batch). `spawn` is commonly used in combination with [`.with`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.with) and [`.with_bundle`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.with_bundle), to stick on extra components or component bundles to the entity you just told the engine to create.
2. Despawning entities with [`despawn`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.despawn) and ['despawn_recursive](https://docs.rs/bevy/0.4.0/bevy/transform/hierarchy/trait.DespawnRecursiveExt.html#tymethod.despawn_recursive), for when you want to despawn all of the children too.
3. Adding components to existing entities with [`insert_one`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert) (for a single component) [`insert`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert) (for more than one component).
4. Removing a component from an existing component with [`remove`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.remove).

You can also add [resources](resources.md) at runtime with [`insert_resource`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_resource) and `insert_local_resource`)https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_local_resource).
This is probably not what you want to do, due to the fact that only one resource of a specific type can exist at a time, the delay in processing commands, and the fact that [`AppBuilder::init_resource<T>`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.init_resource) probably does what you're trying to accomplish better.

## Custom Commands

But all this has barely scratched the truly unlimited power of `Commands`!
With the help of the ['add_command'](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html?search=#method.add_command) and ['add_command_boxed`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html?search=#method.add_command_boxed), we can create our own custom commands that run *any* static function that satisfies the [`Command`](https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Command.html) trait the next time commands are processed.
This trait is extremely flexible: all that it requires is that we have a `write` method that can modify both the `World` and `Resources`.

Be very careful with this pattern, and use it extremely sparingly. It's much less clear than most other ways you could accomplish most tasks, has delayed effect, is probably very slow due to poor data access, can't be run in parallel with other work, and has unlimited global write access to the entire app state. 
However, if you end up with a problem that involves read/write access to a large number of types of data at once, this could be the right tool.

Seriously: don't use custom `Commands` unless you're sure you have no other choice.
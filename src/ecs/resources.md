# Resources

When the data you're looking to store isn't associated with any particular entity, you can stick into a convenient [`Resource`]((https://docs.rs/bevy/0.4.0/bevy/ecs/trait.Resource.html)).
**Resources** are global singletons, accessed by their type, which can be used to store global state.
You might want to use resources for storing and configuring settings, handling a complex data structure like a player's inventory that doesn't fit naturally into the ECS model, or tracking game state like the player's score.

You can use virtually any Rust type as a resource, but if possible, you're going to want your resources to be thread-safe: `'static` lifetime and `Send + Sync`.

## Creating Resources

Assuming that we're working with a thread-safe resource that isn't system local, there are two different ways we can add resources to our app.

When you're working with your [`AppBuilder`](../internals/app-builder.md) (including through a [plugin](../../organization/plugins.md)), there are two ways to add resources:

1. [`init_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.init_resource), which adds a resource of the type specified in its type parameter, with a starting value given by its `Default` trait.

2. [`.add_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_resource), which sets a custom starting value for that type.

Use `.init_resource` when you're planning to set it in a later system (perhaps because you need more complex logic), and use `.add_resource` when you have a good starting value.

If you need to add or overwrite Resources at run-time, consider using [commands](commands.md) with the [.insert_resource](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_resource) method, which works the same as `.add_resource` above. Be mindful though: commands don't take effect until the end of each stage. Most of the time, you shouldn't need to do this: to modify a resource, instead create a system that gets a `ResMut` to the resource in question, then modify it within that system.

Here's how you might add resources of various types for a mock RTS game:
```rust
{{#include resources_code/examples/adding_resources.rs}}
```

## Using Resources in Your Systems

In order to access resources in a system, wrap the resource type in your function parameters in one of several smart-pointers.

1. [`Res`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Res.html), for when you want read-only access to the underlying data.
   
2. [`ResMut`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.ResMut.html), for when you want read and write access to the data.
   
3. [`Local`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Local.html), for when you want a system-local resource.

4. ['ChangedRes'](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.ChangedRes.html), for when you only want your system to run when that resource has been changed this tick.

These resource smart pointers all `impl Deref`, ensuring that rather than needing to call `*my_resource` each time, you can usually implicitly skip the dereferencing with `my_resource`. 

When you define a system, you can include resources as one of your function parameter. Bevy's scheduler automatically looks for a  previously added resources with a matching type, and passes in a reference of the appropriate type to your system.

We can see the differences between these different resource types in this simple example:

```rust
{{#include resources_code/examples/resource_smart_pointers.rs}}
```

## Ensuring Unique Resource Types

When any of the resource creation methods is called on a type that already exists (with the caveat that system-local resources are effectively scoped), Bevy will overwrite any existing data. As a result, you only ever want to have one resource of a given type in your app at once.

While there are a number of different ways to manage differentiate your types in Rust, you're typically going to want to use either the newtype pattern (for resources / components that only incidentally share the same data) or the generic type pattern (for when you have true variants on the same data):

```rust
{{#include resources_code/examples/unique_resource_types.rs}}
```

### Thread-local Resources

If you need a resource that is not thread-safe, you first need to create it with: [`.add_thread_local_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.add_thread_local_resource) or [`.init_thread_local_resource`](https://docs.rs/bevy/0.4.0/bevy/app/struct.AppBuilder.html#method.init_thread_local_resource), whose behavior corresponds to the `add_resource` and `init_resource` methods described above. 

You might need to use resources that aren't thread-safe when:

- interfacing with a scripting language like Lua that isn't thread-safe
- manipulating reference-counted objects
- handling audio-processing, network-queues or other complex data structures that are hard to make thread-safe

Be aware: thread-local resources created in this way are a completely distinct concept from those created with the `.insert_local_resource` method, which use the `Local` resource smart pointer, which creates a unique instantiation of the resource in the system it is referred to.

Once you have your thread-local resource, you need to use "thread-local systems" (see the corresponding [section](../systems.md) in this book for more information) to manipulate it, which gives you a complete global lock on the entire [app](https://docs.rs/bevy/0.4.0/bevy/app/struct.App.html), with `World` and all of its `Resources`.

You can see how you might use thread-local resources in [thread-local systems](../systems.md).

### System-Local Resources

System-local resources are scoped, mutable resources that are only available in the system that created them. Their state persists between time steps, but not between distinct systems created using the same function, as they work off of the `SystemId` created at their time of registration.

In typical use, system-local resources are created implicitly, through the use of a `Local` resource smart-pointer type on one of the function arguments in your system. If you had some reason to manually create or overwrite them, you could instead use [`.insert_local_resource`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.Commands.html#method.insert_local_resource).

Local resources are a fairly niche tool: use them if you want to track state that will only ever be relevant to a single system that you need to persist across frames. The [official example](https://github.com/bevyengine/bevy/blob/master/examples/ecs/event.rs) for [Events](events.md) show how you can use them to create an`EventReader`, which could persist across frames if not all events were processed in the available time. When you want to have many similar, but distinct systems, you can use system-local resources in combination with generics to get interesting specialization, as shown in this overly-complex `Timer` example:
```rust
{{#include resources_code/examples/system_local_resources.rs}}
```

### Detecting Changes to Resources

Much like with components, you can watch for changes to resources with [`ChangedRes`](https://docs.rs/bevy/0.4.0/bevy/ecs/struct.ChangedRes.html).
Systems with a `ChangedRes` parameter only run when the resource has been changed earlier in the current tick.
```rust ```

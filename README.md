# Todo App (WORK IN PROGRESS)

Tiny demonstration of a todo App in Rust with encapsulated layers and microservices to make the app
scalable. The project is organized in sub crates and organized per cargo workspace. The API is
classical CRUD.

## Folder Description

### shared

The shared sub crate contains code that is shared between all crate works like a conversion layer
between the framework and the application i.e. the `NanoServiceError` and other glue code.

### nanoservice

The nanoservice directory hosts nano services like todo api and the authentication api.

## Nano Service Architecture

A nano service is build with a 3 layer architecture core, dal and networking. The idea is to
encapsulate them in different layers to be able to replace them easily without touching the other
layers. If you want to support another web framework you can just add it to the networking layer DAL
and Core stay untouched. By using cargo features it is ensured to compile only layer features that
are actually used and not all of them to keep the binary small.

### Core Layer

The core layer contains the application logic it can be configured to per features flags.

### DAL Layer

This layer encapsulates data access which can be configured by feature flags. So far are planned to
support json-file storage for early development and testing as well as Postgres for production. But
more features can easily be added by adding a feature flag and write module for the feature flag.

### Networking Layer

Here the web frameworks are encapsulated. I will use for now Actix Web, but you can easily support
any other framework just create a sub crate and add a feature flag for in the shared folder and
write a conversion code for the feature.

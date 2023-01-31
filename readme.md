This repo has just finish its first ever iteration. I use this and some small personal projects to experiment with spair.
Spair has no community and no _styleguides or best practices_, so, I leave the rest of this README unchange as provided by the starter kit. Just some lines to provide first looks:

* `/types` is copied from [this](https://github.com/jetli/rust-yew-realworld-example-app). Practically, it can be use for both backend and frontend.
* `/services` has the same origin as `/types`, but I modified this to experiement with `gloo-net` as well as avoid some clones.
* `/lib` the real source code for the Realworld App is here, I made this because I don't know how to use `trunk` to build the app with feature flag.
* `/realworld_gloo_net` and `/realworld_reqwest` which are powered by `gloo-net` and `reqwest` respectively. You can `cd` to this and run `trunk build --release`.

# ![RealWorld Example App](logo.png)

> ### [YOUR_FRAMEWORK] codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.


### [Demo](https://github.com/gothinkster/realworld)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)


This codebase was created to demonstrate a fully fledged fullstack application built with **[YOUR_FRAMEWORK]** including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the **[YOUR_FRAMEWORK]** community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.


# How it works

> Describe the general architecture of your app here

# Getting started

> npm install, npm start, etc.


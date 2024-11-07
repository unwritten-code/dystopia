# backend

## Shuttle
* Use shuttle.dev [documentation](https://docs.shuttle.dev/getting-started/installation), not older shuttle.rs documentation.
* To create a new project run `shuttle login` then `shuttle init`, choose `A Hello World app in a supported framework` and Axum.

## Rust 101
* To run a project locally by
```
cd /backend
shuttle run
```

* To deploy code to production run `shuttle deploy` (can use --ad flag if you haven't committed your code)


# WIP (Learnings)

* xslxwriter depends on libclang library which is not part of shuttle [therefore] use umya_spreadsheet
* https://docs.shuttle.rs/configuration/environment
* This tutorial is really helpful: https://www.youtube.com/watch?v=lowVW7Wa0nI
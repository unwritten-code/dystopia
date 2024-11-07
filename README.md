# Backend

## Shuttle Deployment and Setup
Please use shuttle.dev [documentation](https://docs.shuttle.dev/getting-started/installation) not older shuttle.rs documentation.

### Initial Setup
1. **Login to Shuttle**
Before starting, log in to Shuttle using
```bash
shuttle login
```

2. **Create a New Project:
Once logged in, create a new Shuttle project by running
```bash
shuttle init
```
When prompted, choose the option:
*  `A Hello World app in a supported framework`
* Select `Axum` for the framework

This will set up the project with teh necessary files and dependencies.

## Local Development
To run your backend project locally, follow these steps:
1. Navigate to the `backend` directory:
```bash
cd /backend
```
2. Start the local development server:
```bash
shuttle run
```

## Deploying to Production
To deploy your changes to production, run the following command:
```bash
shuttle deploy
```
If you have uncommitted changes that you wish to deploy, you can use the `--ad` flag
```bash
shuttle deploy --ad
```

## WIP (Learnings)

* xslxwriter depends on libclang library which is not part of shuttle [therefore] use umya_spreadsheet
* https://docs.shuttle.rs/configuration/environment
* This tutorial is really helpful: https://www.youtube.com/watch?v=lowVW7Wa0nI
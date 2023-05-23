# The Easy Tracker Website

This directory contains the source code for the public-facing Easy Tracker website; a website written in the [TypeScript](https://www.typescriptlang.org/) programming language with the [React](https://react.dev/) library.

## Installation Guide

Before installing, make sure you have the latest version of [npm](https://www.npmjs.com/) installed.

1. If you have not already done so, clone the repository and navigate to the website directory.
2. To start a local development server, run the command `npm run dev`.
3. To create a production release, run the command `npm run build`.

## Routing

As the website is a single page application (SPA), client-side routing is needed to enable multiple "pages". To achieve this, we use a client-side routing library called [React Router](https://reactrouter.com/). React Router uses declarative routing and will conditionally render components based on the URL.

## Dependencies

**react** - A JavaScript library for creating user interfaces.
<br />
**react-dom** - A React package for working with the DOM.
<br />
**react-router-dom** - A fully-featured routing library for the React JavaScript library.

# The Easy Tracker Website

This directory contains the source code for the public-facing Easy Tracker website; a website written in the [TypeScript](https://www.typescriptlang.org/) programming language with the [React](https://react.dev/) library.

## Installation Guide

Before installing, make sure you have the latest version of [npm](https://www.npmjs.com/) installed.

1. If you have not already done so, clone the repository and navigate to the website directory.
2. To start a local development server, run the command `npm run dev`.
3. To create a production release, run the command `npm run build`.

## Routing

As the website is a single page application (SPA), client-side routing is needed to enable multiple "pages". To achieve this, we use a client-side routing library called [React Router](https://reactrouter.com/). React Router uses declarative routing and will conditionally render components based on the URL.

## State Management

The website uses [React Query](https://tanstack.com/query/latest/docs/react/overview) to facilitate data fetching, caching, and synchronization.

One of the best features of React Query is the development tools. The development tools allow you to view and manipulate the website's state with an interface on the website. When a production bundle is created, the development interface will be removed.

## Testing

The website uses [Vitest](https://vitest.dev/) for testing. Vitest is a [Vite-native](https://vitejs.dev/) unit testing framework. Along with Vitest, the [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/) is used to test rendering components without a graphical environment and allows us to perform assertions on the rendered components.

Test files should be placed in the same directory as the code they are testing. The test file should have the same name, with `.test` added as a prefix to the file extension. For example, if we are writing test cases for `App.tsx`, we would write them in `App.test.tsx`.

There are two scripts that are related to testing:

1. The command `npm run test` will run all of the test cases.
2. The command `npm run coverage` will create a report about the website's testing coverage.

## Dependencies

**@hookform/resolvers** - External validation library support for React Hook Form.
<br />
**@tanstack/react-query** - An asynchronous state management solution for JavaScript.
<br />
**@tanstack/react-query-devtools** - Development tools for React Query (_not included in the production bundle_).
<br />
**axios** - A promise-based HTTP client.
<br />
**react** - A JavaScript library for creating user interfaces.
<br />
**react-dom** - A React package for working with the DOM.
<br />
**react-hook-form** - A performant, flexible and extensible form library with easy-to-use validation.
<br />
**react-router-dom** - A fully-featured routing library for the React JavaScript library.
<br />
**zod** - A TypeScript-first schema validation library with static type inference.

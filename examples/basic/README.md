# Yewi
 **Yewi-cli** app template inspired by **React / Next.ts architecture**, featuring:
- Component-local folders
- Separated logic into custom hooks
- Styling with `tailwind-css` + `scss` (using `@apply` to avoid messy and unreusable inline classes)

**Yewi** is a custom **yew** project mirroring the **Next.js** project structure and development experience.

## Getting started

To start the development server, run:
``` bash
trunk serve
```
Open [http://localhost:8081](http://localhost:8081) with your browser to see the result.
You can start editing the page by modifying `src/app/page.rs` or add components from [Yewi-kit](https://yewi.fiaro.app) by running:

``` bash
yewi add button
yewi add input avatar card
# Or
yewi add
# Then select the components you want to add
```

## Learn more

To learn more about **Yewi-kit**, check the following resources:
- [Yewi-kit-Documentation](https://yewi.fiaro.app): for installation and components usage guide
- [Yewi-kit-Repository](https://github.com/Emii-lia/yewi-kit): Components source code with preview examples

To get the list of available components, run the command:

``` bash
yewi list
```

Or check the [documentation](https://yewi.pages.dev/docs)

## Folder structure

- `src`: Source directory
  - `components/` : All pieces level components
  - `features/` : Contains all views, i.e. components using pieces level components
  - `app/` : Contains the pages and routes of the app (similar to next app routes)
  - `app/routes.rs` : Contains the routes' definition
  - `app/mod.rs` : Contains the main layout of the app (similar to `app/layout.tsx`)
  - `styles/` : Contains global styles and styles importation
  - `styles/components.scss` : Imports all `components/` styles
  - `styles/main.scss` : Global styles
  - `styles/features.scss` : Imports all `features/` styles
  - `types/` : All custom types
  - `main.rs` : Entry point of the app

## Styling with Tailwind and Scss
Each component, features and pages have their own `*.scss` file ; and these files are imported in the global `components.scss`, `features.scss` respectively (you can add `pages.scss` if you want).
You can use tailwindcss `@apply` to use tailwindcss classes inside `scss` file

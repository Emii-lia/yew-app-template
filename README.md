# Yew app template
An **App template for Yew** inspired by **React / Next.ts architecture**, featuring:
- Component-local folders
- Separated logic into custom hooks
- Styling with `tailwind-css` + `scss` (using `@apply` to avoid messy and unreusable inline classes)

## Component Structure

| Concern              | Location                        |
| -------------------- |---------------------------------|
| View (JSX / HTML)    | `mod.rs`                        |
| Logic (custom hooks) | `hooks.rs`                      |
| Utilities            | `(types/data/utils/contants).rs` |
| Styles               | `style.scss`                    |

## Folder structure

- `src`: Source directory
  - `components/` : All pieces level components
  - `features/` : Contains all views, i.e. components using pieces level components
  - `app/` : Contains the pages and routes of the app (similar to next app routes)
  - `app/routes` : Contains the routes' definition
  - `app/mod.rs` : Contains the main layout of the app (similar to `app/layout.tsx`)
  - `styles/` : Contains global styles and styles importation
  - `styles/components.scss` : Imports all `components/` styles
  - `styles/main.scss` : Global styles
  - `styles/features.scss` : Imports all `features/` styles
  - `types/` : All custom types
  - `utils/` : Contains all custom utils
  - `main.rs` : Entry point of the app

## Styling with Tailwind
Each component, features and pages have their own `*.scss` file ; and these files are imported in the global `components.scss`, `features.scss` respectively (you can add `pages.scss` if you want).

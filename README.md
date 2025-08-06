A re-imagining of the user showcase on the https://dioxus.dev/ website.

- Run `dx serve` to start the development server.

- Run `npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch` to compile Tailwind styles.

- Run `dx build --release` to build for release.

- After building, you must manually add the `link` tags for loading CSS into the `index.html` file in the `public` folder. Otherwise there will be a flash of unstyled content on page load.

Tastemaker Design is not affiliated with Dioxus Labs. The Dioxus name and logo are the intellectual property of Dioxus Labs, Inc.

% id: 12
% title: A lightweight stack for a simple web app
% date: 2024-01-30
% tags: web

When I started doing web development about five years ago I started using frontend frameworks such as React right of the bat. Especially then, single page applications was the go-to paradigm. Without knowing much about html, css and javascript, it was rather easy to get started with React. React kind of forces you to split code into reusable components, where each component is contained in a single file, bundling the html, css (if you're using something like styled-components or tailwind css) and javascript (or typescript). Each component can contain state and each component can render other components. This allows for deeply nested component trees, which can make state management complex, because you might have to transfer state between nodes that are far apart. For such projects, I ended up using Redux, a centralized state management pattern. This whole journey was quite fun but I started to create a feeling that I was studying a tool rather than a concept. Also, the React repository started to become bloated and it became harder to navigate.

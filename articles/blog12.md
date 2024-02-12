% id: 12
% title: A lightweight stack for a simple web app

## On the client side

The problem I recently ran into was that I wanted to apply search engine optimization (SEO) techniques on my single-page React application. This proved to be very challenging, as the HTML that represented my application was rendered completely on the client side. This introduced some issues for search engine crawlers, such as, but not limited to:

1. Crawlers process static HTML content. I believe that nowadays, crawlers are more advanced and can execute the javascript that renders the HTML and process that. However, if a search engine doesn't do this (for a number of reasons) a lot of content won't be available.
2. Crawlers aim for fast loading times and can't be bothered by heavy javascript bundles. Delays negatively affect indexing of search engines.
3. When the SPA uses client-side routing, hash routes are used to prevent a page reload. This causes an unconventional sitemap which can be misunderstood by the crawler of a search engine, leaving a part of the SPA undiscovered.

To tackle these issues, I decided to serve HTML content directly from the server (server-side rendering). On top of this I used [htmx](https://htmx.org/essays/hypermedia-driven-applications/) for frontend interactivity. This is my first time using htmx but I can already see the benefits of a [hypermedia-driven application](https://hypermedia.systems/hypermedia-reintroduction/)([HDA](https://htmx.org/essays/hypermedia-driven-applications/)). htmx is a javascript library that has no depedencies and can be added to a project with a simple `<script>` tag. It's also declaratively embedded in HTML, making it pretty concise and easy to read. As an example, check out this htmx-embedded-HTML snippet that makes a POST request to a search endpoint on the server, using the input value as search parameter:

```html
<input
  id="search-input"
  type="search"
  placeholder="Search..."
  hx-target="#search-results"
  hx-trigger="input changed delay:500ms, search"
  hx-post="/search"
/>
<div id="search-results"></div>
```

This will wait half a second after the last time the user has typed something before making the request. The response data, which is HTML, will be rendered within the `<div>` at the bottom.

The following snippet is the JSX (React) equivalent of the previous htmx snippet, generated with ChatGPT. I could've made the translation myself, but it wouldn't make a difference in making a point: htmx is much more concise, even if you would heavily refactor this JSX snippet.

```jsx
import React, { useState, useEffect } from "react";

const SearchComponent = () => {
  const [query, setQuery] = useState("");
  const [searchResults, setSearchResults] = useState([]);

  useEffect(() => {
    const timer = setTimeout(() => {
      // Perform search when query changes
      search(query);
    }, 500);

    return () => clearTimeout(timer);
  }, [query]);

  const handleInputChange = (event) => {
    setQuery(event.target.value);
  };

  const search = async (searchQuery) => {
    try {
      const response = await fetch("/search", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ query: searchQuery }),
      });
      const data = await response.json();
      setSearchResults(data.results);
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <>
      <input
        id="search-input"
        type="search"
        placeholder="Search..."
        value={query}
        onChange={handleInputChange}
      />
      <div id="search-results">
        {searchResults.map((result, index) => (
          <div key={index}>{result}</div>
        ))}
      </div>
    </>
  );
};
```

Also, the hypermedia that is shipped by the server is much more sensible to search engine crawlers than minified HTML, which would be the case with SPA (React SPA in this example). On the other hand, minified HTML has the benefit of being smaller in size and increasing overall site speed.

## On the server side

After having used several server side technologies I wanted to try something new. Since some time [rust](https://www.rust-lang.org/) has become my preferred language for side-projects because its type system helps me to prevent making mistakes and when I make mistakes, the compiler usually knows how to pinpoint the problem. Also, it provides a way to get more experienced with memory management, as opposed to dynamic interpreted languages like Python where this trait is less apparent. Anyways, I decided to use [axum](https://github.com/tokio-rs/axum), a web framework written in rust that supposedly is very modular because it uses [tower](https://docs.rs/tower/latest/tower/trait.Service.html) middleware, not its own, and because it seems ergonomic, looking at the minimal examples in [the documentation](https://github.com/tokio-rs/axum?tab=readme-ov-file). Not that loading hypermedia would ever take long, but axum also seems performant, as its only a thin wrapper around the low-level HTTP implementation for rust. See the [performance benchmark](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md) for yourself.

To make the application more extendable, I used [askama](https://github.com/djc/askama), a type-safe template engine that uses syntax similar to jinja. Type-safety is enforced through user defined structs. For example, this is a blog struct that fills in the `blog.html` template, which is situated (and expected by default) in `templates/`.

```rust
use askama::Template;
use sqlx::types::time::Date;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    body: String,
    date: Date,
}
```

Where `blog.html` has placeholders for the struct fields. The example also reveals the possibility to include other templates, in this case a footer. This would still allow you to create [a component tree](https://legacy.reactjs.org/docs/higher-order-components.html) as is often seen in React projects. At last, I'm explicitly telling the compiler that `body` is `safe`, as it contains HTML, which might [not be a very good idea](https://owasp.org/www-project-web-security-testing-guide/latest/4-Web_Application_Security_Testing/11-Client-side_Testing/03-Testing_for_HTML_Injection), but that's something for later and not so critical for an example blog page.

```html
<div
  class="flex justify-center flex-col gap-4 pt-4 px-4 max-w-screen-md mx-auto sm:pt-24"
>
  <header class="font-mono text-2xl text-gray-900 font-black">
    {{ title }}
  </header>
  <div class="font-bold font-mono text-gray-900">{{ date }}</div>
  <div class="font-mono text-gray-900 w-full">{{ body|safe }}</div>
  {% include "footer.html" %}
</div>
```

The way that I used it is with a `templates` folder that contains a number of HTML files, for each route and even components in routes, such as a header and footer.

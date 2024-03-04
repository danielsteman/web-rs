% id: 12
% title: A lightweight stack for a simple web app
% date: 2024-02-13
% tags: web

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

## Styling

For styling I used TailwindCSS. It works really well together with htmx because at this point most client-side logic and styling is embedded in the HTML served by Axum, making the resulting code concise. Setting up Tailwind was as straight forward as `yarn add tailwindcss`. For development and production runs I added to following scripts to my `package.json`.

```json
"scripts": {
  "dev": "npx tailwindcss -i ./templates/input.css -o ./assets/output.css --watch",
  "prod": "npx tailwindcss -i ./templates/input.css -o ./assets/output.css --minify"
}
```

Tailwind utilities are imported in `input.css`. It's possible to append styling rules to this file, as they will be included in the generated output.

```css
@import "tailwindcss/base";
@import "tailwindcss/components";
@import "tailwindcss/utilities";

/*
As an example of custom styles on top of tailwind
that applies to h2 headers that are rendered by markdown.rs
*/

body[class*="blog"] h2 {
  padding: 32px 0px 8px 0px;
  font-weight: 700;
}
```

## On the server side

After having used several server side technologies I wanted to try something new. Since some time [rust](https://www.rust-lang.org/) has become my preferred language for side-projects because its type system helps me to prevent making mistakes and when I make mistakes, the compiler usually knows how to pinpoint the problem. Also, it provides a way to get more experienced with memory management, as opposed to dynamic interpreted languages like Python where this trait is less apparent. Anyways, I decided to use [axum](https://github.com/tokio-rs/axum), a web framework written in rust that supposedly is very modular because it uses [tower](https://docs.rs/tower/latest/tower/trait.Service.html) middleware, not its own, and because it seems ergonomic, looking at the minimal examples in [the documentation](https://github.com/tokio-rs/axum?tab=readme-ov-file). Not that loading hypermedia would ever take long, but axum also seems performant, as its only a thin wrapper around the low-level HTTP implementation for rust. See the [performance benchmark](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md) for yourself.

## Templates

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

## Database

A content management system is usually overkill for a simple project and in case of my blog, I basically write articles in markdown and commit them. Transforming markdown to HTML is made easy with the `to_html()` method of [markdown.rs](). To prevent that each markdown file needs to be transformed to HTML everytime the blogs page is loaded, I decided to setup a postgres database. A simple ingestion script performs the job of parsing the markdown files in `/articles/` and pushing articles to the database.

To interact with the database, I have used `sqlx`. This package makes it easy to make migrations with the `sqlx::migrate!()` macro. By default, it expects SQL scripts to be stored in `/migrations/` with a number prefix (`_1`) such that it's clear in which order the migrations need to be applied. For example, ensuring that the `blog` table exists in the postgres database the application is connected to, is as easy as have a file, `migrations/1_blog.sql`, with the query below.

```sql
CREATE TABLE blog (
    id INT4 PRIMARY KEY,
    title TEXT,
    summary TEXT,
    body TEXT,
    date DATE,
    tags TEXT[]
);
```

The documentation of `sqlx` is quite clear and implementing a struct that gets blogs from the database was straight forward. The only thing that was tricky was matching rust types with postgres types. For example, `INT4` in postgres maps to `i32` in rust and I explicitly needed to create the `id` column with the `INT4` type for the application to work with some managed cloud instance of postgres. If you're using a different database, this is something keep in mind.

```rs
#[derive(PartialEq, Debug, sqlx::FromRow)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub date: Date,
    pub tags: Vec<String>,
}

impl Blog {
    pub async fn get_blogs(pool: &Pool<Postgres>) -> Result<Vec<Blog>, Error> {
        let mut blogs: Vec<Blog> = sqlx::query_as::<_, Blog>("SELECT * FROM blog")
            .fetch_all(pool)
            .await?;

        blogs.sort_by(|a, b| b.id.cmp(&a.id));

        Ok(blogs)
    }
}
```

The whole development experience that I got from building this example was pleasant and refreshing. The fact that most client-side code is embedded in HTML makes the project concise and ergonomic. I'm not sure if htmx will be the answer for larger projects that require much more user interaction, as it would be hard to refactor more complex logic. But if you're building a prototype and want to experience something else than a mainstream javascript framework, I can highly recommend this stack.

This stack also tackles the SEO issues I had before, because all content is now available without executing javascript, but also because it follows a conventional sitemap and loads pages quickly. It must be noted that these tools have worked well for me in this project but that they might not be the best choice for another project. As a wise person once said: "learn concepts, not tools".

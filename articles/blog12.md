% id: 12
% title: A lightweight stack for a simple web app

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

% id: 12
% title: A lightweight stack for a simple web app

The problem I recently ran into was that I wanted to apply search engine optimization (SEO) techniques on my single-page React application. This proved to be very challenging, as the HTML that represented my application was rendered completely on the client side. This introduced some issues for search engine crawlers, such as, but not limited to:

1. Crawlers process static HTML content. I believe that nowadays, crawlers are more advanced and can execute the javascript that renders the HTML and process that. However, if a search engine doesn't do this (for a number of reasons) a lot of content won't be available.
2. Crawlers aim for fast loading times and can't be bothered by heavy javascript bundles. Delays negatively affect indexing of search engines.
3. When the SPA uses client-side routing, hash routes are used to prevent a page reload. This causes an unconventional sitemap which can be misunderstood by the crawler of a search engine, leaving a part of the SPA undiscovered.

To tackle these issues, I decided to serve HTML content directly from the server (server-side rendering). On top of this I used [htmx](https://htmx.org/essays/hypermedia-driven-applications/) for frontend interactivity. This is my first time using htmx but I can already see the benefits. htmx is a javascript library that has no depedencies and can be added to a project with a simple `<script>` tag. It's also declaratively embedded in HTML, making it pretty concise and easy to read. As an example, check out this input element that makes a POST request to the server, using the input as form data:

```
<input
    id="search-input"
    type="search"
    name="search_string"
    placeholder="Search..."
    hx-target="#search-results"
    hx-trigger="input changed delay:500ms, search"
    hx-post="/search"
/>
```

% id: 12
% title: A lightweight stack for a simple web app
% date: 2024-01-30
% tags: web

The problem I recently ran into was that I wanted to apply search engine optimization (SEO) techniques on my single-page React application. This proved to be very challenging, as the HTML that represented my application was rendered completely on the client side. This introduced some issues for search engine crawlers:

1. Crawlers process static HTML content. I believe that nowadays, crawlers are more advanced and can execute the javascript that renders the HTML and process that. However, if a search engine doesn't do this (for a number of reasons) a lot of content won't be available.
2. Crawlers aim for fast loading times and can't be bothered by heavy javascript bundles. Delays negatively affect indexing of search engines.
3.

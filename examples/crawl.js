// Example usage of BlessCrawl plugin

async function main() {
  const crawler = new BlessCrawl();
  try {
    console.log("=== BlessCrawl Example ===\n");

    console.log("1. Scraping webpage...");
    const scrapeResult = await crawler.scrape("https://example.com", {
      format: "markdown",
    });
    console.log(JSON.stringify(scrapeResult, null, 2));

    console.log("2. Mapping links...");
    const mapResult = await crawler.map("https://example.com");
    console.log(JSON.stringify(mapResult, null, 2));

    console.log("3. Crawling website...");
    const crawlResult = await crawler.crawl("https://example.com", {
      max_depth: 2,           // Only go 2 levels deep
      limit: 10,              // Maximum 10 pages
      follow_external: false, // Don't follow external links
      delay_between_requests: 1000, // 1 second delay between requests
      parallel_requests: 2    // Max 2 parallel requests
    });
    console.log(JSON.stringify(crawlResult, null, 2));
  } catch (error) {
    console.error("Error:", error);
  }
}

main()
  .then(() => console.log("\n=== Example completed ==="))
  .catch((error) => console.error("Example failed:", error)); 

// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __BlessCrawl = globalThis.BlessCrawl;

    // BlessCrawl class wrapper
    class BlessCrawl {
        constructor(config = {}) {
            // Create the underlying crawl instance through the Rust binding
            this._instance = __BlessCrawl(config);
        }

        /**
         * Scrape webpage content and return as markdown with metadata
         * @param {string} url - The URL to scrape
         * @param {Object} options - Optional mapping options
         * @returns {Promise<Object>} - Promise that resolves to scrape response
         */
        async scrape(url, options = {}) {
            if (typeof url !== 'string') {
                throw new Error('URL must be a string');
            }
            return new Promise((resolve, reject) => {
                try {
                    const result = this._instance.scrape(url, options);
                    if (result.success) {
                        resolve(result.data);
                    } else {
                        reject(result.error);
                    }
                } catch (error) {
                    reject(error);
                }
            });
        }

        /**
         * Extract all links from a webpage, categorized by type
         * @param {string} url - The URL to map
         * @param {Object} options - Optional mapping options
         * @returns {Promise<Object>} - Promise that resolves to map response
         */
        async map(url, options = {}) {
            if (typeof url !== 'string') {
                throw new Error('URL must be a string');
            }
            return new Promise((resolve, reject) => {
                try {
                    const result = this._instance.map(url, options);
                    if (result.success) {
                        resolve(result.data);
                    } else {
                        reject(result.error);
                    }
                } catch (error) {
                    reject(error);
                }
            });
        }

        /**
         * Recursively crawl a website with configurable depth and filtering
         * @param {string} url - The URL to start crawling from
         * @param {Object} options - Optional crawl options
         * @returns {Promise<Object>} - Promise that resolves to crawl response
         */
        async crawl(url, options = {}) {
            if (typeof url !== 'string') {
                throw new Error('URL must be a string');
            }
            return new Promise((resolve, reject) => {
                try {
                    const result = this._instance.crawl(url, options);
                    if (result.success) {
                        resolve(result.data);
                    } else {
                        reject(result.error);
                    }
                } catch (error) {
                    reject(error);
                }
            });
        }
    }

    // Expose the BlessCrawl class globally
    globalThis.BlessCrawl = BlessCrawl;

    // Delete the internal function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__BlessCrawl");
})(); 

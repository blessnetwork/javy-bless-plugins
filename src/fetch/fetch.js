// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_fetchio_request = globalThis.__javy_fetchio_request;

    function fetch(url, options = {}) {
        // const encodedOutput = new TextEncoder().encode(JSON.stringify(options))
        // const data = new Uint8Array(encodedOutput)
        const parsedOptions = JSON.stringify(options)

        const responseObj = __javy_fetchio_request(url, parsedOptions);

        // @TODO: Capture all response data from response object
        const responseOk = responseObj.ok;
        const responseHeaders = {};
        const responseBody = responseObj.body;

        return new Promise((resolve, reject) => {
            const response = {
                url,
                headers: responseHeaders,
                ok: responseOk,
                type: typeof responseBody === 'string' ? 'text' : 'json',
                text: () => typeof responseBody === 'string' ? responseBody : JSON.stringify(responseBody),
                json: () => typeof responseBody !== 'string' ? responseBody : JSON.parse(responseBody),
            };

            resolve(response);
        });
    }

    globalThis.fetch = fetch

    // Delete the function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_fetchio_request");
})();
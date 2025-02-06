// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_crypto_get_random_values = globalThis.__javy_crypto_get_random_values;

    function getRandomValues(data) {
        __javy_crypto_get_random_values(data.buffer, data.byteOffset, data.byteLength)
        return new Uint8Array(data.buffer)
    }

    globalThis.crypto = {
        getRandomValues
    }

    // Delete the function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_crypto_get_random_values");
})();
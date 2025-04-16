(function () {
    const __encode = globalThis.__javy_b64_encode;
    const __decode = globalThis.__javy_b64_decode;

    globalThis.btoa = function (input) {
        const encoder = new TextEncoder();
        const buffer = encoder.encode(input);
        return __encode(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    };

    globalThis.atob = function (b64) {
        const decoded = __decode(b64);
        return new TextDecoder().decode(decoded);
    };

    Reflect.deleteProperty(globalThis, "__javy_b64_encode");
    Reflect.deleteProperty(globalThis, "__javy_b64_decode");
})();
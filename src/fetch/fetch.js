// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_fetchio_request = globalThis.__javy_fetchio_request;

    class Headers {
        constructor(init) {
            this._headers = {};
            if (init) {
                if (init instanceof Headers) {
                    // Copy from another Headers instance
                    for (const [key, value] of init.entries()) {
                        this.append(key, value);
                    }
                } else if (Array.isArray(init)) {
                    // Array of [key, value] pairs
                    for (const pair of init) {
                        if (pair.length !== 2) {
                            throw new TypeError('Invalid headers array');
                        }
                        this.append(pair[0], pair[1]);
                    }
                } else if (typeof init === 'object') {
                    // Object literal
                    for (const key of Object.keys(init)) {
                        this.append(key, init[key]);
                    }
                }
            }
        }

        append(name, value) {
            name = this._normalizeHeaderName(name);
            value = String(value);
            
            if (this._headers[name]) {
                this._headers[name] += ', ' + value;
            } else {
                this._headers[name] = value;
            }
        }

        delete(name) {
            name = this._normalizeHeaderName(name);
            delete this._headers[name];
        }

        get(name) {
            name = this._normalizeHeaderName(name);
            return this._headers[name] || null;
        }

        has(name) {
            name = this._normalizeHeaderName(name);
            return name in this._headers;
        }

        set(name, value) {
            name = this._normalizeHeaderName(name);
            this._headers[name] = String(value);
        }

        *entries() {
            for (const key of Object.keys(this._headers)) {
                yield [key, this._headers[key]];
            }
        }

        *keys() {
            for (const key of Object.keys(this._headers)) {
                yield key;
            }
        }

        *values() {
            for (const value of Object.values(this._headers)) {
                yield value;
            }
        }

        forEach(callback, thisArg) {
            for (const [key, value] of this.entries()) {
                callback.call(thisArg, value, key, this);
            }
        }

        _normalizeHeaderName(name) {
            return String(name).toLowerCase();
        }

        // Convert to plain object for passing to Rust
        _toObject() {
            return { ...this._headers };
        }
    }

    class Request {
        constructor(input, init = {}) {
            if (input instanceof Request) {
                // Clone from another Request
                this._url = input.url;
                this._method = init.method || input.method;
                this._headers = new Headers(init.headers || input.headers);
                this._body = init.body !== undefined ? init.body : input._body;
                this._mode = init.mode || input.mode;
                this._credentials = init.credentials || input.credentials;
                this._cache = init.cache || input.cache;
                this._redirect = init.redirect || input.redirect;
                this._referrer = init.referrer || input.referrer;
                this._referrerPolicy = init.referrerPolicy || input.referrerPolicy;
                this._integrity = init.integrity || input.integrity;
                this._keepalive = init.keepalive !== undefined ? init.keepalive : input.keepalive;
                this._signal = init.signal || input.signal;
            } else {
                // Create from URL
                this._url = String(input);
                this._method = init.method || 'GET';
                this._headers = new Headers(init.headers);
                this._body = init.body;
                this._mode = init.mode || 'cors';
                this._credentials = init.credentials || 'same-origin';
                this._cache = init.cache || 'default';
                this._redirect = init.redirect || 'follow';
                this._referrer = init.referrer || 'about:client';
                this._referrerPolicy = init.referrerPolicy || '';
                this._integrity = init.integrity || '';
                this._keepalive = init.keepalive || false;
                this._signal = init.signal;
            }

            // Validate
            if (this._body && (this._method === 'GET' || this._method === 'HEAD')) {
                throw new TypeError('Request with GET/HEAD method cannot have body');
            }
        }

        get url() { return this._url; }
        get method() { return this._method; }
        get headers() { return this._headers; }
        get mode() { return this._mode; }
        get credentials() { return this._credentials; }
        get cache() { return this._cache; }
        get redirect() { return this._redirect; }
        get referrer() { return this._referrer; }
        get referrerPolicy() { return this._referrerPolicy; }
        get integrity() { return this._integrity; }
        get keepalive() { return this._keepalive; }
        get signal() { return this._signal; }
        clone() {
            return new Request(this);
        }
    }

    class Response {
        constructor(body, init = {}) {
            this._body = body;
            this._status = init.status || 200;
            this._statusText = init.statusText || '';
            this._headers = new Headers(init.headers);
            this._url = init.url || '';
            this._redirected = init.redirected || false;
            this._type = init.type || 'basic';
            this._ok = this._status >= 200 && this._status < 300;
            this._bodyUsed = false;
        }

        static _fromNative(nativeResponse) {
            const response = new Response(nativeResponse.body || null);

            // Copy properties from native response
            response._status = nativeResponse.status;
            response._statusText = nativeResponse.statusText;
            response._ok = nativeResponse.ok;
            response._url = nativeResponse.url;
            response._redirected = nativeResponse.redirected;
            response._type = nativeResponse.type;

            // Convert headers object to Headers instance
            response._headers = new Headers(nativeResponse.headers);

            // Bind native methods
            response._nativeText = nativeResponse.text;
            response._nativeJson = nativeResponse.json;
            response._nativeArrayBuffer = nativeResponse.arrayBuffer;
            response._nativeBlob = nativeResponse.blob;
            response._nativeClone = nativeResponse.clone;

            return response;
        }

        get ok() { return this._ok; }
        get status() { return this._status; }
        get statusText() { return this._statusText; }
        get headers() { return this._headers; }
        get url() { return this._url; }
        get redirected() { return this._redirected; }
        get type() { return this._type; }
        get bodyUsed() { return this._bodyUsed; }

        async text() {
            if (this._bodyUsed) {
                throw new TypeError('Body already consumed');
            }
            this._bodyUsed = true;
            
            if (this._nativeText) {
                return this._nativeText();
            }
            return String(this._body || '');
        }

        async json() {
            if (this._bodyUsed) {
                throw new TypeError('Body already consumed');
            }
            this._bodyUsed = true;
            if (this._nativeJson) {
                return this._nativeJson();
            }
            const text = await this.text();
            return JSON.parse(text);
        }

        async arrayBuffer() {
            if (this._bodyUsed) {
                throw new TypeError('Body already consumed');
            }
            this._bodyUsed = true;
            
            if (this._nativeArrayBuffer) {
                return this._nativeArrayBuffer();
            }
            
            // Simplified implementation
            const text = await this.text();
            const encoder = new TextEncoder();
            return encoder.encode(text).buffer;
        }

        async blob() {
            if (this._bodyUsed) {
                throw new TypeError('Body already consumed');
            }
            this._bodyUsed = true;

            if (this._nativeBlob) {
                return this._nativeBlob();
            }

            // Return blob-like object
            const arrayBuffer = await this.arrayBuffer();
            return {
                size: arrayBuffer.byteLength,
                type: this.headers.get('content-type') || 'application/octet-stream',
                arrayBuffer: () => Promise.resolve(arrayBuffer),
                text: () => this.text(),
            };
        }

        clone() {
            if (this._bodyUsed) {
                throw new TypeError('Cannot clone a response with used body');
            }

            // If this response came from native code (has native methods), call native clone
            if (this._nativeClone) {
                return Response._fromNative(this._nativeClone());
            }

            // Otherwise create a JavaScript clone
            const clonedResponse = new Response(this._body, {
                status: this._status,
                statusText: this._statusText,
                headers: this._headers,
                url: this._url,
                redirected: this._redirected,
                type: this._type,
            });
            
            // Copy native methods if they exist
            if (this._nativeText) clonedResponse._nativeText = this._nativeText;
            if (this._nativeJson) clonedResponse._nativeJson = this._nativeJson;
            if (this._nativeArrayBuffer) clonedResponse._nativeArrayBuffer = this._nativeArrayBuffer;
            if (this._nativeBlob) clonedResponse._nativeBlob = this._nativeBlob;

            return clonedResponse;
        }

        static error() {
            const response = new Response(null, { status: 0, statusText: '' });
            response._type = 'error';
            return response;
        }

        static redirect(url, status = 302) {
            if (![301, 302, 303, 307, 308].includes(status)) {
                throw new RangeError('Invalid redirect status');
            }
            const headers = new Headers({ Location: url });
            return new Response(null, { status, headers });
        }
    }

    // FormData class implementation
    class FormData {
        constructor() {
            this._entries = [];
            this._isFormData = true;
        }

        append(name, value, filename) {
            this._entries.push([String(name), String(value), filename]);
        }

        delete(name) {
            name = String(name);
            this._entries = this._entries.filter(entry => entry[0] !== name);
        }

        get(name) {
            name = String(name);
            const entry = this._entries.find(entry => entry[0] === name);
            return entry ? entry[1] : null;
        }

        getAll(name) {
            name = String(name);
            return this._entries
                .filter(entry => entry[0] === name)
                .map(entry => entry[1]);
        }

        has(name) {
            name = String(name);
            return this._entries.some(entry => entry[0] === name);
        }

        set(name, value, filename) {
            name = String(name);
            this.delete(name);
            this.append(name, value, filename);
        }

        *entries() {
            for (const [name, value] of this._entries) {
                yield [name, value];
            }
        }

        *keys() {
            for (const [name] of this._entries) {
                yield name;
            }
        }

        *values() {
            for (const [, value] of this._entries) {
                yield value;
            }
        }

        forEach(callback, thisArg) {
            for (const [name, value] of this.entries()) {
                callback.call(thisArg, value, name, this);
            }
        }
    }

    // Main fetch function
    async function fetch(input, init = {}) {
        let url;
        let options = {};

        if (input instanceof Request) {
            url = input.url;
            options = {
                method: input.method,
                headers: input.headers._toObject(),
                body: input._body,
                mode: input.mode,
                credentials: input.credentials,
                cache: input.cache,
                redirect: input.redirect,
                referrer: input.referrer,
                referrerPolicy: input.referrerPolicy,
                integrity: input.integrity,
                keepalive: input.keepalive,
                signal: input.signal,
                ...init, // init overrides
            };
        } else {
            url = String(input);
            options = {
                method: 'GET',
                ...init,
            };
        }

        // Convert headers to object if Headers instance
        if (options.headers instanceof Headers) {
            options.headers = options.headers._toObject();
        }

        // Handle body
        if (options.body) {
            if (typeof options.body === 'string') {
                // String body - pass as is
            } else if (options.body instanceof FormData) {
                // FormData - convert to format expected by Rust
                options.body = {
                    _isFormData: true,
                    _entries: options.body._entries.map(([name, value]) => [name, value]),
                };
            } else if (ArrayBuffer.isView(options.body) || options.body instanceof ArrayBuffer) {
                // Binary data - convert to array
                const view = new Uint8Array(options.body);
                options.body = Array.from(view);
            } else if (typeof options.body === 'object') {
                // JSON - stringify
                options.body = JSON.stringify(options.body);
                if (!options.headers) {
                    options.headers = {};
                }
                if (!options.headers['content-type'] && !options.headers['Content-Type']) {
                    options.headers['Content-Type'] = 'application/json';
                }
            }
        }

        try {
            // Call native fetch function
            const nativeResponse = __javy_fetchio_request(url, options);
            // Convert native response to Response instance
            return Response._fromNative(nativeResponse);
        } catch (error) {
            // Network errors should return a network error response
            if (error.message && error.message.includes('network')) {
                return Response.error();
            }
            throw error;
        }
    }

    // Expose global APIs
    globalThis.fetch = fetch;
    globalThis.Headers = Headers;
    globalThis.Request = Request;
    globalThis.Response = Response;
    globalThis.FormData = FormData;

    // Delete the native function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_fetchio_request");
})();

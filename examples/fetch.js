// Example demonstrating the fetch-compliant wrapper around http_v2 client

console.log("🚀 Fetch Demo - Browser-compatible fetch API");
console.log("============================================");

async function main() {
    try {
        // Example 1: Simple GET request
        console.log("\n1. Simple GET request:");
        const response1 = await fetch("https://httpbin.org/get");
        console.log(`Status: ${response1.status} ${response1.statusText}`);
        console.log(`OK: ${response1.ok}`);
        
        const data1 = await response1.json();
        console.log(`URL: ${data1.url}`);
        console.log(`User-Agent: ${data1.headers['User-Agent']}`);

        // Example 2: POST with JSON body
        console.log("\n2. POST with JSON body:");
        const postData = {
            name: "Blockless Network",
            type: "Decentralized Computing",
            version: "2.0",
            fetch_api: "standard"
        };
        const response2 = await fetch("https://httpbin.org/post", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "X-Custom-Header": "fetch-demo"
            },
            body: JSON.stringify(postData)
        });

        console.log(`POST Status: ${response2.status}`);
        const postResult = await response2.json();
        console.log(`Received JSON: ${JSON.stringify(postResult.json)}`);
        console.log(`Custom Header: ${postResult.headers['X-Custom-Header']}`);

        // Example 3: Headers API
        console.log("\n3. Headers API demonstration:");
        const headers = new Headers();
        headers.append("Authorization", "Bearer token123");
        headers.append("Accept", "application/json");
        headers.set("User-Agent", "Blockless-Fetch/1.0");

        const response3 = await fetch("https://httpbin.org/headers", {
            method: "GET",
            headers: headers
        });

        const headersData = await response3.json();
        console.log("Sent headers:", headersData.headers);

        // Example 4: Request object
        console.log("\n4. Request object demonstration:");
        const request = new Request("https://httpbin.org/user-agent", {
            method: "GET",
            headers: {
                "User-Agent": "Blockless-SDK-Fetch/1.0"
            }
        });

        console.log(`Request URL: ${request.url}`);
        console.log(`Request Method: ${request.method}`);

        const response4 = await fetch(request);
        const userAgentData = await response4.json();
        console.log(`Detected User-Agent: ${userAgentData['user-agent']}`);

        // Example 5: PUT request with text body
        console.log("\n5. PUT request with text body:");
        const response5 = await fetch("https://httpbin.org/put", {
            method: "PUT",
            headers: {
                "Content-Type": "text/plain"
            },
            body: "This is plain text data from fetch!"
        });

        const putData = await response5.json();
        console.log(`PUT Data received: ${putData.data}`);

        // Example 6: Error handling
        console.log("\n6. Error handling:");
        try {
            const errorResponse = await fetch("https://httpbin.org/status/404");
            console.log(`Error status: ${errorResponse.status} ${errorResponse.statusText}`);
            console.log(`Is successful: ${errorResponse.ok}`);
            
            if (!errorResponse.ok) {
                console.log("Request failed but didn't throw an error (as per fetch spec)");
            }
        } catch (error) {
            console.log(`Network error: ${error.message}`);
        }

        // Example 7: Response methods
        console.log("\n7. Response methods demonstration:");
        const textResponse = await fetch("https://httpbin.org/robots.txt");
        const textContent = await textResponse.text();
        console.log(`Text content length: ${textContent.length} characters`);
        console.log(`First line: ${textContent.split('\n')[0]}`);

        // Example 8: Multiple headers with same name
        console.log("\n8. Multiple headers:");
        const multiHeaders = new Headers();
        multiHeaders.append("Accept", "application/json");
        multiHeaders.append("Accept", "text/html");
        
        console.log(`Accept header: ${multiHeaders.get("Accept")}`);

        // Example 9: DELETE request
        console.log("\n9. DELETE request:");
        const deleteResponse = await fetch("https://httpbin.org/delete", {
            method: "DELETE",
            headers: {
                "X-Delete-Reason": "Testing fetch implementation"
            }
        });

        console.log(`DELETE Status: ${deleteResponse.status}`);
        const deleteData = await deleteResponse.json();
        console.log(`Headers sent: ${JSON.stringify(deleteData.headers)}`);

        // Example 10: PATCH request
        console.log("\n10. PATCH request:");
        const patchResponse = await fetch("https://httpbin.org/patch", {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ 
                update: "partial update", 
                timestamp: new Date().toISOString(),
                fetch_api: "standard"
            })
        });

        const patchData = await patchResponse.json();
        console.log(`PATCH result: ${JSON.stringify(patchData.json)}`);

        // Example 11: Response cloning
        console.log("\n11. Response cloning demonstration:");
        const originalResponse = await fetch("https://httpbin.org/json");
        
        console.log(`Original response status: ${originalResponse.status}`);
        console.log(`Original response ok: ${originalResponse.ok}`);
        
        // Clone the response before consuming the body
        const clonedResponse = originalResponse.clone();
        
        console.log(`Cloned response status: ${clonedResponse.status}`);
        console.log(`Cloned response ok: ${clonedResponse.ok}`);
        
        // Consume both responses independently
        const originalData = await originalResponse.json();
        const clonedData = await clonedResponse.json();
        
        console.log(`Original slideshow title: ${originalData.slideshow?.title || 'N/A'}`);
        console.log(`Cloned slideshow title: ${clonedData.slideshow?.title || 'N/A'}`);
        console.log(`Both responses consumed successfully: ${JSON.stringify(originalData) === JSON.stringify(clonedData)}`);

        // Example 12: Clone error handling
        console.log("\n12. Clone error handling:");
        try {
            const response = await fetch("https://httpbin.org/get");
            await response.text(); // Consume the body
            
            // This should throw an error
            response.clone();
            console.log("❌ ERROR: Clone should have failed!");
        } catch (error) {
            console.log(`✅ Expected error caught: ${error.message}`);
        }

        // Example 13: Binary data with Uint8Array
        console.log("\n13. Binary data with Uint8Array:");
        const binaryData = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]); // "Hello World"
        const binaryResponse = await fetch("https://httpbin.org/post", {
            method: "POST",
            headers: {
                "Content-Type": "application/octet-stream"
            },
            body: binaryData
        });

        const binaryResult = await binaryResponse.json();
        console.log(`Binary data sent successfully: ${binaryResponse.status}`);
        console.log(`Data received by server: ${binaryResult.data}`);

        // Example 14: ArrayBuffer body
        console.log("\n14. ArrayBuffer body:");
        const buffer = new ArrayBuffer(16);
        const view = new Uint8Array(buffer);
        for (let i = 0; i < view.length; i++) {
            view[i] = i + 65; // ASCII A, B, C, etc.
        }
        
        const bufferResponse = await fetch("https://httpbin.org/post", {
            method: "POST",
            headers: {
                "Content-Type": "application/octet-stream",
                "X-Binary-Type": "ArrayBuffer"
            },
            body: buffer
        });

        const bufferResult = await bufferResponse.json();
        console.log(`ArrayBuffer sent successfully: ${bufferResponse.status}`);
        console.log(`Content-Type header: ${bufferResult.headers['Content-Type']}`);
        console.log(`Custom header: ${bufferResult.headers['X-Binary-Type']}`);

        // Example 15: Response arrayBuffer() method
        console.log("\n15. Response arrayBuffer() method:");
        const imageResponse = await fetch("https://httpbin.org/image/png");
        console.log(`Image response status: ${imageResponse.status}`);
        console.log(`Content-Type: ${imageResponse.headers.get('Content-Type')}`);
        
        const imageBuffer = await imageResponse.arrayBuffer();
        console.log(`Image buffer size: ${imageBuffer.byteLength} bytes`);
        
        // Convert first few bytes to hex for display
        const imageView = new Uint8Array(imageBuffer);
        const hexPreview = Array.from(imageView.slice(0, 16))
            .map(b => b.toString(16).padStart(2, '0'))
            .join(' ');
        console.log(`First 16 bytes (hex): ${hexPreview}`);

        // Example 16: Binary data with regular array (fallback test)
        console.log("\n16. Binary data with regular array:");
        const regularArray = [66, 108, 111, 99, 107, 108, 101, 115, 115]; // "Blockless"
        const arrayResponse = await fetch("https://httpbin.org/post", {
            method: "POST",
            headers: {
                "Content-Type": "application/octet-stream",
                "X-Array-Type": "Regular"
            },
            body: regularArray
        });

        const arrayResult = await arrayResponse.json();
        console.log(`Regular array sent: ${arrayResponse.status}`);
        console.log(`Data type header: ${arrayResult.headers['X-Array-Type']}`);

        console.log("\n✅ All fetch examples completed successfully!");
        
    } catch (error) {
        console.error("❌ Error in fetch demo:", error.message);
        console.error(error.stack || error);
    }
}

// Run the demo
main().then(() => {
    console.log("\n🏁 Demo finished");
}).catch(error => {
    console.error("💥 Demo failed:", error);
});
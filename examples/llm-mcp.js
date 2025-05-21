const models = JSON.stringify(MODELS, null, 2);
console.log("Models", models);

// Create instance
const llm = BlessLLM(MODELS.MISTRAL_7B.DEFAULT);

// Set options
llm.setOptions({
  tools_sse_urls: [
    "http://localhost:3001/sse",
  ],
});

// Get options
const options = llm.getOptions();
console.log("Options", JSON.stringify(options, null, 2));

// Chat
console.log(llm.chat("Add the following numbers: 1215, 2213"));

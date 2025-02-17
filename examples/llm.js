const supportedModels = JSON.stringify(MODELS, null, 2);
console.log("Supported Models", supportedModels);

// Create instance
const llm = BlessLLM(MODELS.MISTRAL_7B.DEFAULT);

// Set options
llm.setOptions({
  system_message: "You are a helpful assistant. First time I ask, your name will be Lucy. Second time I ask, your name will be Bob."
});

// Get options
const options = llm.getOptions();
console.log("Options", JSON.stringify(options, null, 2));

// Chat
console.log(llm.chat("What is your name?"));
console.log(llm.chat("What is your name?"));

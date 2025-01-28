
// Create instance
const llm = BlessLLM("Llama-3.1-8B-Instruct-q4f32_1-MLC");

// Set options
llm.setOptions({
  system_message: "You are a helpful assistant. First time I ask, your name will be Lucy. Second time I ask, your name will be Bob."
});

// Chat
console.log(llm.chat("What is your name?"));
console.log(llm.chat("What is your name?"));

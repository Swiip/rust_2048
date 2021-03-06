import wrap from "/wrap.js";

const run = async () => {
  try {
    const { instance } = await WebAssembly.instantiateStreaming(
      fetch("/wasm/lib_2048.wasm"),
      {}
    );
    console.log("WebAssembly instantiate success", instance, instance.exports);
    const reducer = wrap(instance, "reducer");
    const state = {
      board: { current_id: 0, grid: [] },
      changed: false,
      won: false,
      lost: false
    };
    const action = {
      action_type: "Init",
      direction: null,
      random_value: null,
      random_position: null
    };
    console.log("2048 state and action", state, action);
    const newState = reducer({ state, action });
    console.log("2048 state after Rust reduce", newState);
    console.log("WebAssembly is awesome!");
  } catch (error) {
    console.log("WebAssembly instantiate failed", error);
  }
};

run();

const { invoke } = window.__TAURI__.tauri;

let credsUsernameEl;
let credsPasswordEl;
let credsMsgEl;

async function store_credentials() {
  console.log("creds", credsUsernameEl.value, credsPasswordEl.value)
  credsMsgEl.textContent = await invoke("store_credentials", { username: credsUsernameEl.value, password: credsPasswordEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  credsUsernameEl = document.querySelector("#creds-username");
  credsPasswordEl = document.querySelector("#creds-password");
  credsMsgEl = document.querySelector("#creds-msg");
  document.querySelector("#creds-form").addEventListener("submit", (e) => {
    e.preventDefault();
    store_credentials();
  });
});

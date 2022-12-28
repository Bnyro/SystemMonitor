const { invoke } = window.__TAURI__.tauri;

let $ = document.querySelector.bind(document);

async function get_sys_info() {
  let sysInfo = JSON.parse(await invoke("get_sys_info"));

  Object.keys(sysInfo).forEach(field => {
    if (field.includes("usage")) {
      const progress = parseInt(sysInfo[field] * 100);
      $(`#${field}`).setAttribute("data-progress", progress);
      $(`#${field}`).setAttribute("style", `--progress: ${progress};`);
    } else {
      $(`#${field}`).innerHTML = sysInfo[field];
    }
  })
}

window.addEventListener("DOMContentLoaded", () => {
  get_sys_info();
  window.setInterval(() => get_sys_info(), 1000);
});

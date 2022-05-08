import { invoke } from '@tauri-apps/api/tauri'


console.log("here!")

let btn = document.getElementById("entries");

btn.addEventListener("click", function () {
	window.location.href = "entries.html";
});
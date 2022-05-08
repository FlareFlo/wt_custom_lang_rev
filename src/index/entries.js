import { invoke } from '@tauri-apps/api/tauri'


document.getElementById("test").addEventListener("click", function () {
	invoke("list_all");
});
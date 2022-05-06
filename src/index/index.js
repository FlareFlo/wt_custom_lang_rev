// const invoke = window.__TAURI__.invoke
import { invoke } from '@tauri-apps/api/tauri'


console.log("here!")

let btn = document.getElementById("test_btn");
console.log(btn);

btn.addEventListener("click", function () {
	invoke('my_custom_command')
	alert("sus");
});
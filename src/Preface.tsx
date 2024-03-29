import { open } from "@tauri-apps/api/dialog";
import { appConfigDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api";
export function Preface() {
    function click() {
        invoke("open_main_path_and_save_recent", {}).then((res) => {
            console.log(res);
        });
    }
    return <button onClick={click}>Click</button>;
}

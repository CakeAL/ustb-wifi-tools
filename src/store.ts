import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

const setCurUserName = async () => {
  await invoke("set_current_user_name", { userName: store.userName });
};

export const store = reactive({
  userName: "",
  setUserName(userName: string) {
    this.userName = userName;
    setCurUserName();
  },
  clearUserName() {
    this.userName = "";
    setCurUserName();
  },
});

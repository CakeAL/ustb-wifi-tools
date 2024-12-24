import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

const getCurUserName = async (): Promise<string> => {
    return await invoke("get_current_user_name");
}

const setCurUserName = async () => {
    await invoke("set_current_user_name", {userName: store.userName});  
}

export const store = reactive({
  userName: await getCurUserName(),
  setUserName(userName: string) {
    this.userName = userName;
    setCurUserName();
  },
  clearUserName() {
    this.userName = "";
    setCurUserName();
  }
});

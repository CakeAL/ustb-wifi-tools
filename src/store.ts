import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

const getCurUserName = async (): Promise<string> => {
  return await invoke("get_current_user_name");
};

const setCurUserName = async () => {
  await invoke("set_current_user_name", { userName: store.userName });
};

async function initializeStore() {
  const userName = await getCurUserName();
  return reactive({
    userName,
    setUserName(userName: string) {
      this.userName = userName;
      setCurUserName();
    },
    clearUserName() {
      this.userName = "";
      setCurUserName();
    },
  });
}

export const store = await initializeStore();

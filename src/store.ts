import { reactive } from "vue";

export const store = reactive({
  userName: "",
  setUserName(userName: string) {
    this.userName = userName;
  },
  clearUserName() {
    this.userName = "";
  }
});

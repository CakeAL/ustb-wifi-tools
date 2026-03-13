import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

const setCurUserName = async () => {
  await invoke("set_current_user_name", { userName: store.userName });
};

export const store = reactive({
  userName: "",
  userDashboard: undefined as UserDashboard | undefined,
  setUserName(userName: string) {
    this.userName = userName;
    setCurUserName();
  },
  clearUserName() {
    this.userName = "";
    setCurUserName();
  },
});

export interface UserDashboard {
  accessGrant: string;
  bindCmFlag: string;
  chinanetDownFlow: number; // 这是教育网ipv6下行
  chinanetUpFlow: number; // 这是教育网ipv6上行
  downloadBand: number;
  installDate: string; // 用户首次出现日期
  installLocal: string;
  installmentFlag: number;
  internetDownFlow: number; // ipv4下行
  internetUpFlow: number; // ipv4上行
  invalidDate: number; // 账号到期时间
  ipCount: number;
  leftFlow: number; // 剩余ipv4流量
  leftMoney: number; // 剩余金额
  leftTime: number;
  localId: number;
  macAddress: string; // MAC地址（4个）
  multiFlag: number;
  multiGroupId: number;
  multiLogin: number;
  otherFlow: number;
  payStyle: number;
  serviceDefault: ServiceDefault;
  serviceString: string;
  specialLine: number;
  specialServiceFlag: string;
  startAdminId: number;
  startDate: number;
  startDelay: number;
  startType: number;
  stopAdminId: number;
  stopDate: string;
  stopReason: string;
  uploadBand: number;
  useFlag: number;
  useFlow: number;
  useMoney: number;
  useTime: number;
  userExtar: UserExtra;
  userGroup: UserGroup;
  userGroupId: number;
  userId: number;
  userIdNumber: string; // 这里校园网实际上使用身份证作为id
  userIdType: string; // 应为身份证
  userIp: string;
  userName: string; // 学号
  userPassword: string; // 这个没用
  userRealName: string;
  vlanId: number;
}

interface ServiceDefault {
  areaId: number;
  code: string;
  defaultName: string; // 用户类别
  extend: string; // 额外信息
  id: number;
  preRegiste: number;
  specialServiceFlag: string;
  userGroupId: number;
}

interface UserExtra {
  userId: number;
}

interface UserGroup {
  allRate: number;
  bandIp: number;
  bandMac: number;
  baseCycle: number;
  baseMoney: number;
  flowControlFlag: number;
  flowRate: number; // 超出流量每MB多少元
  flowStart: number; // 当前可用总流量
  groupBand: number;
  groupMaxFlow: number;
  groupMaxTime: number;
  ipCount: number;
  ipMaxCount: number;
  limitFlag: number;
  limitMoney: number;
  multiFlag: number;
  payFirstFlag: number;
  payStyle: number;
  setfeesId: number;
  timeControlFlag: number;
  timeRate: number;
  timeStart: number;
  userBand: number;
  userGroupDescription: string;
  userGroupId: number;
  userGroupName: string;
  userMaxFlow: number;
  userMaxTime: number;
  weekConfig: number;
}

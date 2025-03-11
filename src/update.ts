import { Channel, invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

export const is_download = ref(false);
export const download_percent = ref(0);

type DownloadEvent =
  | {
    event: "started";
    data: {
      newVersion: boolean;
    };
  }
  | {
    event: "progress";
    data: {
      downloaded: number;
      contentLength: number;
    };
  }
  | {
    event: "finished";
    data: {
      finished: boolean;
    };
  };

const onEvent = new Channel<DownloadEvent>();
onEvent.onmessage = (message) => {
  if (message.event === "started") {
    is_download.value = message.data.newVersion; // 这没有意义
  } else if (message.event === "progress") {
    download_percent.value = parseFloat(
      (message.data.downloaded / message.data.contentLength).toFixed(2),
    ) * 100;
  }
};

export const check_update = async (manually: boolean) => {
  await invoke("manually_check_update", {
    manually,
    onEvent,
  }).catch((err) => console.log(err));
};

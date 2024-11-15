import { CSSProperties } from "vue";

export const railStyle = ({
  focused,
  checked,
}: {
  focused: boolean;
  checked: boolean;
}) => {
  const style: CSSProperties = {};
  if (checked) {
    style.background = "#4b9e5f";
    if (focused) {
      style.boxShadow = "0 0 0 2px #dbecdfff";
    }
  } else {
    style.background = "#2080f0";
    if (focused) {
      style.boxShadow = "0 0 0 2px #2080f040";
    }
  }
  return style;
};

export const mb2gb = (mb: number | undefined) => {
  return parseFloat(((mb as number) / 1024).toFixed(2));
};

export const min2hour = (min: number | undefined) => {
  return parseFloat(((min as number) / 60).toFixed(2));
};

export const min2day = (min: number | undefined) => {
  return parseFloat(((min as number) / 60 / 24).toFixed(2));
};

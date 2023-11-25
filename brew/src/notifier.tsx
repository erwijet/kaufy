import useWS from "react-use-websocket";
import toast from "react-hot-toast";
import { pipe } from "@tsly/core";
import { useUserStore } from "./utils/auth";
import { maybe } from "@tsly/maybe";

export const Notifier = () => {
  const id = useUserStore((s) => s.info?.id);

  useWS(
    `wss://ntfy.kaufy.holewinski.dev/${
      maybe(id)?.take((it) => "usr" + it) ?? "-"
    }/ws`,
    {
      onMessage(msg) {
        pipe(JSON.parse(msg.data), (data) => {
          if (data.event == "message") {
            toast(data.message, { icon: data.title });
          }
        });
      },
    },
  );

  return <></>;
};

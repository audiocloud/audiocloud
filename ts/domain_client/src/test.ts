import {
  InstancePlayState,
  InstancePowerState,
  InstanceSpec,
} from "./instance";
import { createRtcSocket } from "./rtc_socket";

async function main() {
  const [sendEvents] = createRtcSocket("http://127.0.0.1:7200", {
    connectionChanged(connected: boolean): any {
      console.log("connected", connected);
    },
    instanceConnectionChanged(instanceId: string, connected: boolean) {
      console.log("instance connected", instanceId, connected);
    },
    instancePlayStateChanged(instanceId: string, state: InstancePlayState) {
      console.log("instance play state changed", instanceId, state);
    },
    instancePowerStateChanged(instanceId: string, state: InstancePowerState) {
      console.log("instance power state changed", instanceId, state);
    },
    instanceReport(
      instance: string,
      name: string,
      channel: number,
      value: number
    ) {
      console.log("instance report", instance, name, channel, value);
    },
    instanceSpec(instanceId: string, spec: InstanceSpec | null) {
      console.log("instance spec", instanceId, spec);
    },
  });

  setTimeout(() => sendEvents.close(), 10000);
}

main()
  .then(() => console.log("done"))
  .catch((e) => console.error(e));

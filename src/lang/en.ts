import type { Dictionary } from "./types";

export const en: Dictionary = {
  appName: "Sito Node Ports",
  consentTitle: "Put me in the menu bar?",
  consentText:
    "Sito Node Ports lives next to the clock. Click its icon to see every Node process with an open port, labelled with its project name.",
  consentHint:
    "The window will hide and the app will leave the Dock. You can remove it from here at any time.",
  shortcutHint: (shortcut: string) =>
    `Can't see the icon? Press ${shortcut} from any app, or reopen Sito Node Ports from Spotlight or Launchpad.`,
  consentAccept: "Yes, put it in the menu bar",
  trayActive: "Active in the menu bar",
  trayDisable: "Remove from the menu bar",
  hideWindow: "Hide window",
  processesTitle: "Node processes with ports",
  emptyTitle: "Nothing listening",
  emptyHint: "No Node processes have open TCP ports right now.",
  openPort: (port: number) => `Open localhost:${port}`,
  pidLabel: (pid: number) => `PID ${pid}`,
  kill: "Terminate process",
  dismiss: "Close",
  errorProcessNotFound: "That process is no longer listening.",
  errorKillFailed: "Couldn't terminate the process.",
  errorGeneric: "Something went wrong:",
};

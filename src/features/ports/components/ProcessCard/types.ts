import { NodeProcess } from "@/shared/models";

export interface ProcessCardProps {
  process: NodeProcess;
  busy: boolean;
  onOpen: (port: number) => void;
  onKill: (pid: number) => void;
}

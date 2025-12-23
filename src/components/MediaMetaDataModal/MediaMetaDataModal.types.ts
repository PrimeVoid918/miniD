import type { BaseMedia } from "../../lib/services/tuari-command/tuari-command.types";

export interface MediaMetaDataModalProps {
  open: boolean;
  onClose: () => void;
  media: BaseMedia | null;
}

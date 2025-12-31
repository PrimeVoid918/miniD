import type { Media } from "../../lib/services/tuari-command/tuari-command.types";

export interface MediaMetaDataModalProps {
  open: boolean;
  onClose: () => void;
  media: Media | null;
}

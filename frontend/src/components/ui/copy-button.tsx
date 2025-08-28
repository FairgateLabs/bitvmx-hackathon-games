import { Copy } from "lucide-react";
import { Button } from "@/components/ui/button";

interface CopyButtonProps {
  text: string;
  disabled?: boolean;
  size?: "sm" | "default" | "lg";
  variant?:
    | "outline"
    | "default"
    | "destructive"
    | "secondary"
    | "ghost"
    | "link";
  className?: string;
  onCopy?: () => void;
}

export function CopyButton({
  text,
  disabled = false,
  size = "sm",
  variant = "outline",
  className = "",
  onCopy,
}: CopyButtonProps) {
  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(text);
      onCopy?.();
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
    }
  };

  return (
    <Button
      size={size}
      variant={variant}
      onClick={copyToClipboard}
      disabled={disabled || !text}
      className={className}
    >
      <Copy className="h-4 w-4" />
    </Button>
  );
}

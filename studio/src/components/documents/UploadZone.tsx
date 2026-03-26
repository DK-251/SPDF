import { useState, useCallback, useRef } from "react";
import { Upload, FileUp } from "lucide-react";
import { cn } from "@/lib/utils";

interface UploadZoneProps {
  onFileSelect: (file: File) => void;
  accept?: string;
  className?: string;
}

export function UploadZone({
  onFileSelect,
  accept = ".spdf",
  className,
}: UploadZoneProps) {
  const [isDragOver, setIsDragOver] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      setIsDragOver(false);
      const file = e.dataTransfer.files[0];
      if (file) onFileSelect(file);
    },
    [onFileSelect],
  );

  const handleClick = () => {
    inputRef.current?.click();
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) onFileSelect(file);
    e.target.value = "";
  };

  return (
    <div
      role="button"
      tabIndex={0}
      onClick={handleClick}
      onKeyDown={(e) => e.key === "Enter" && handleClick()}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      className={cn(
        "flex cursor-pointer flex-col items-center justify-center gap-3 rounded-xl border-2 border-dashed p-8 transition-all",
        isDragOver
          ? "border-primary bg-primary/5 text-primary"
          : "border-border text-muted-foreground hover:border-zinc-600 hover:text-foreground",
        className,
      )}
    >
      {isDragOver ? (
        <FileUp className="h-8 w-8 animate-bounce" />
      ) : (
        <Upload className="h-8 w-8" />
      )}
      <div className="text-center">
        <p className="text-sm font-medium">
          {isDragOver ? "Drop to upload" : "Drop an SPDF file here"}
        </p>
        <p className="mt-1 text-xs text-muted-foreground">
          or click to browse
        </p>
      </div>
      <input
        ref={inputRef}
        type="file"
        accept={accept}
        onChange={handleChange}
        className="hidden"
        aria-label="Upload file"
      />
    </div>
  );
}

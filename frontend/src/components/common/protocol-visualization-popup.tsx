import { X } from "lucide-react";
import { Button } from "@/components/ui/button";

interface ProtocolVisualizationPopupProps {
  isOpen: boolean;
  onClose: () => void;
  visualization: SVGSVGElement | null;
  isLoading?: boolean;
}

export function ProtocolVisualizationPopup({
  isOpen,
  onClose,
  visualization,
  isLoading = false,
}: ProtocolVisualizationPopupProps) {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center p-4">
      <div className="bg-white rounded-lg shadow-xl w-full h-full max-w-none max-h-none flex flex-col">
        {/* Header with color legend */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">
            Protocol Visualization
          </h2>
          <Button
            variant="ghost"
            size="sm"
            onClick={onClose}
            className="hover:bg-gray-100"
          >
            <X className="h-5 w-5" />
          </Button>
        </div>
        <div className="p-4 ">
          <div className="text-gray-600">
            <span className="text-green-600"> Green</span> - Seen in the
            blockchain
            <br />
            <span className="text-black"> Black</span> - Not seen in the
            blockchain
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-auto p-4">
          {isLoading ? (
            <div className="flex items-center justify-center h-full">
              <div className="text-center">
                <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                <p className="text-gray-600">
                  Loading protocol visualization...
                </p>
              </div>
            </div>
          ) : visualization ? (
            <div className="w-full h-full flex items-center justify-center">
              <div
                className="w-full h-full overflow-auto"
                dangerouslySetInnerHTML={{
                  __html: visualization.outerHTML,
                }}
              />
            </div>
          ) : (
            <div className="flex items-center justify-center h-full">
              <div className="text-center">
                <p className="text-gray-600">No visualization available</p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

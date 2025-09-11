import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { CopyButton } from "@/components/ui/copy-button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useFundingUtxos, useSaveFundingUtxos } from "@/hooks/useFundingUtxos";
import { EnumPlayerRole } from "@/types/game";
import { Utxo } from "../../../../backend/bindings/Utxo";
import { useCurrentGame } from "@/hooks/useGame";

export function UtxoExchange() {
  const [isOpen, setIsOpen] = useState(true);
  const [otherUtxo, setOtherUtxo] = useState<Partial<Utxo>>({
    txid: "",
    vout: 0,
    amount: BigInt(0),
  });
  const [jsonInput, setJsonInput] = useState("");
  const [successMessage, setSuccessMessage] = useState("");
  const [jsonError, setJsonError] = useState("");

  const { data: currentGame, isLoading: isGameLoading } = useCurrentGame();
  const role = currentGame?.role;
  const {
    data: fundingUtxos,
    isLoading: isFundingLoading,
    error: fundingUtxoError,
  } = useFundingUtxos(currentGame?.program_id || "");
  const { mutate: saveFundingUtxos, isPending: isSavingUtxo } =
    useSaveFundingUtxos();

  const isValidTxid = (txid: string): boolean => {
    const hexRegex = /^[0-9a-fA-F]{64}$/;
    return hexRegex.test(txid);
  };

  const isValidVout = (vout: string): boolean => {
    const num = parseInt(vout, 10);
    return !isNaN(num) && num >= 0;
  };

  const isValidAmount = (amount: bigint): boolean => {
    return amount > BigInt(0);
  };

  const handleJsonPaste = (jsonString: string) => {
    setJsonInput(jsonString);
    setJsonError("");

    try {
      const parsed = JSON.parse(jsonString);

      if (
        parsed.txid &&
        parsed.vout !== undefined &&
        parsed.amount !== undefined
      ) {
        const utxo: Utxo = {
          txid: parsed.txid,
          vout: parsed.vout,
          amount: parsed.amount,
          output_type: parsed.output_type || {},
        };

        setOtherUtxo(utxo);
        setJsonError("");
      } else {
        setJsonError(
          "Invalid UTXO format. Missing required fields: txid, vout, amount"
        );
      }
    } catch (error) {
      setJsonError("Invalid JSON format");
    }
  };

  const handleSendOtherUtxo = () => {
    if (isOtherUtxoValid() && currentGame?.program_id) {
      saveFundingUtxos(
        { uuid: currentGame.program_id, otherUtxo: otherUtxo as Utxo },
        {
          onSuccess: () => {
            setSuccessMessage("Other participant's UTXO sent successfully!");
          },
          onError: (error) => {
            console.error("Failed to send other UTXO:", error);
          },
        }
      );
    }
  };

  const getMyUtxoJson = () => {
    if (!fundingUtxos) return "";
    console.log("fundingUtxos", fundingUtxos);
    return JSON.stringify(fundingUtxos, null, 2);
  };

  const isOtherUtxoValid = () => {
    return (
      otherUtxo.txid &&
      isValidTxid(otherUtxo.txid) &&
      otherUtxo.vout !== undefined &&
      otherUtxo.amount !== undefined &&
      isValidAmount(otherUtxo.amount)
    );
  };

  return (
    <div className="p-4 border rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 cursor-pointer">
            💰 Bitcoin UTXO Exchange
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm mb-4">
            {role === EnumPlayerRole.Player1
              ? "Share your UTXO information with Player 2 and enter their UTXO details."
              : "Share your UTXO information with Player 1 and enter their UTXO details."}
          </p>

          {/* My UTXO Information */}
          {fundingUtxos && (
            <>
              <div className="p-4 rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <h4 className="font-semibold mb-3">
                    Funding Protocol and Bet UTXO
                  </h4>
                  <CopyButton
                    text={getMyUtxoJson()}
                    size="sm"
                    variant="outline"
                  >
                    Copy to Share
                  </CopyButton>
                </div>

                <pre className="w-full h-50 p-2 text-xs font-mono border rounded resize-none overflow-auto">
                  {JSON.stringify(JSON.parse(getMyUtxoJson()), null, 2)}
                </pre>
              </div>
            </>
          )}

          <div className="border-t border-gray-200 my-4" />

          {/* Other Player's UTXO Input */}
          <div className="p-4">
            <h4 className="font-semibold mb-3">
              Other Player's Protocol and Bet UTXO Information
            </h4>

            <div className="mb-4">
              <textarea
                value={jsonInput}
                rows={8}
                onChange={(e) => handleJsonPaste(e.target.value)}
                placeholder='Paste JSON here, e.g., {"txid":"123...","vout":0,"amount":1000,"output_type":{}}'
                className="w-full h-20 p-2 text-xs font-mono border rounded resize-none"
              />
              {jsonError && (
                <p className="text-sm mt-1 text-red-600">{jsonError}</p>
              )}
              {!jsonError && otherUtxo.txid && (
                <p className="text-sm mt-1 text-green-600">
                  ✅ Valid UTXO JSON
                </p>
              )}
            </div>

            <Button
              onClick={handleSendOtherUtxo}
              disabled={!isOtherUtxoValid() || isSavingUtxo}
              className="w-full"
            >
              {isSavingUtxo ? "Saving..." : "📤 Send Other Player's UTXO"}
            </Button>
          </div>

          {successMessage && (
            <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
              <h4 className="font-semibold text-green-800">✅ Success</h4>
              <p className="text-sm text-green-700">{successMessage}</p>
            </div>
          )}

          {!successMessage && (
            <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h4 className="font-semibold text-yellow-800">
                ⚠️ UTXO Exchange Required
              </h4>
              <p className="text-sm text-yellow-700">
                Both players need to create and share their UTXO information to
                proceed with the game.
              </p>
            </div>
          )}
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}

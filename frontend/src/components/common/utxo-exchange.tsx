import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { CopyButton } from "@/components/ui/copy-button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useMyFundingUtxo, useSendOtherUtxo } from "@/hooks/useUtxo";
import { useGameRole } from "@/hooks/useGameRole";
import { PlayerRole } from "@/types/game";
import { Utxo } from "../../../../backend/bindings/Utxo";
import { useCurrentGame } from "@/hooks/useGame";

interface UtxoExchangeProps {
  gameId: string | null;
}

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
  const { data: role } = useGameRole();
  const {
    data: myUtxo,
    isLoading: isMyUtxoLoading,
    error: myUtxoError,
  } = useMyFundingUtxo(currentGame?.program_id || "");
  const { mutate: sendOtherUtxo, isPending: isSendingUtxo } =
    useSendOtherUtxo();

  const isValidTxid = (txid: string): boolean => {
    // Bitcoin transaction ID is 64 hex characters
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

      // Validate the JSON structure matches Utxo
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
      sendOtherUtxo(
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
    if (!myUtxo) return "";
    return JSON.stringify(myUtxo, null, 2);
  };

  const handleOtherUtxoChange = (field: keyof Utxo, value: string) => {
    setOtherUtxo((prev: Partial<Utxo>) => ({
      ...prev,
      [field]:
        field === "vout"
          ? parseInt(value) || 0
          : field === "amount"
          ? BigInt(parseFloat(value) || 0)
          : value,
    }));
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
            {role === PlayerRole.Player1
              ? "Share your UTXO information with Player 2 and enter their UTXO details."
              : "Share your UTXO information with Player 1 and enter their UTXO details."}
          </p>

          <div className="space-y-4">
            {/* My UTXO Information */}
            <div className="p-4  rounded-lg">
              <h4 className="font-semibold mb-3">My Funding UTXO</h4>
              {isMyUtxoLoading ? (
                <p className="text-sm">Loading my funding UTXO...</p>
              ) : myUtxoError ? (
                <p className="text-sm">
                  Failed to load my funding UTXO: {myUtxoError.message}
                </p>
              ) : myUtxo ? (
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <Label>Transaction ID:</Label>
                    <div className="flex items-center gap-2">
                      <p className="font-mono text-sm p-2 rounded max-w-[300px] truncate">
                        {myUtxo.txid}
                      </p>
                      <CopyButton
                        text={myUtxo.txid}
                        size="sm"
                        variant="outline"
                      />
                    </div>
                  </div>
                  <div className="flex items-center justify-between">
                    <Label>Output Index:</Label>
                    <p className="font-mono text-sm">{myUtxo.vout}</p>
                  </div>
                  <div className="flex items-center justify-between">
                    <Label>Amount:</Label>
                    <p className="font-mono text-sm">
                      {myUtxo.amount.toString()} satoshis
                    </p>
                  </div>

                  {/* My UTXO JSON Display and Copy */}
                  <div className="mt-4 p-3 border rounded-lg">
                    <div className="flex items-center justify-between mb-2">
                      <Label className="font-semibold">My UTXO (JSON):</Label>
                      <CopyButton
                        text={getMyUtxoJson()}
                        size="sm"
                        variant="outline"
                      />
                    </div>
                    <pre className="text-xs p-2 rounded border overflow-x-auto">
                      {getMyUtxoJson()}
                    </pre>
                  </div>
                </div>
              ) : (
                <p className="text-sm">No funding UTXO available</p>
              )}
            </div>

            <div className="border-t border-gray-200 my-4"></div>

            {/* Other Player's UTXO Input */}
            <div className="p-4">
              <h4 className="font-semibold mb-3">
                Other Player's UTXO Information
              </h4>

              {/* JSON Input Area */}
              <div className="mb-4">
                <Label className="font-semibold mb-2 block">
                  Paste Other Player's UTXO JSON:
                </Label>
                <textarea
                  value={jsonInput}
                  onChange={(e) => handleJsonPaste(e.target.value)}
                  placeholder='Paste JSON here, e.g., {"txid":"123...","vout":0,"amount":1000,"output_type":{}}'
                  className="w-full h-20 p-2 text-xs font-mono border rounded resize-none"
                />
                {jsonError && <p className="text-sm mt-1">{jsonError}</p>}
                {!jsonError && otherUtxo.txid && (
                  <p className="text-sm mt-1">✅ Valid UTXO JSON</p>
                )}
              </div>

              {/* <div className="space-y-3">
                <div>
                  <Label htmlFor="otherTxid">Transaction ID (txid):</Label>
                  <Input
                    id="otherTxid"
                    value={otherUtxo.txid || ""}
                    onChange={(e) =>
                      handleOtherUtxoChange("txid", e.target.value)
                    }
                    placeholder="e.g., 1234567890abcdef..."
                    className="mt-1 font-mono text-sm"
                  />
                  {otherUtxo.txid && !isValidTxid(otherUtxo.txid) && (
                    <p className="text-sm">
                      Invalid transaction ID format (64 hex characters)
                    </p>
                  )}
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <Label htmlFor="otherVout">Output Index (vout):</Label>
                    <Input
                      id="otherVout"
                      type="number"
                      value={otherUtxo.vout || ""}
                      onChange={(e) =>
                        handleOtherUtxoChange("vout", e.target.value)
                      }
                      placeholder="0"
                      className="mt-1"
                    />
                    {otherUtxo.vout !== undefined &&
                      !isValidVout(otherUtxo.vout.toString()) && (
                        <p className="text-sm">Invalid output index</p>
                      )}
                  </div>

                  <div>
                    <Label htmlFor="otherAmount">Amount (satoshis):</Label>
                    <Input
                      id="otherAmount"
                      type="number"
                      value={
                        otherUtxo.amount ? otherUtxo.amount.toString() : ""
                      }
                      onChange={(e) =>
                        handleOtherUtxoChange("amount", e.target.value)
                      }
                      placeholder="1000"
                      className="mt-1"
                    />
                    {otherUtxo.amount !== undefined &&
                      !isValidAmount(otherUtxo.amount) && (
                        <p className="text-sm">Amount must be greater than 0</p>
                      )}
                  </div>
                </div>

                <div className="space-y-2">
                  <Button
                    onClick={handleSendOtherUtxo}
                    disabled={!isOtherUtxoValid() || isSendingUtxo}
                    className="w-full"
                  >
                    {isSendingUtxo
                      ? "Sending..."
                      : "📤 Send Other Player's UTXO"}
                  </Button>
                </div>
              </div> */}
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
                  Both players need to create and share their UTXO information
                  to proceed with the game.
                </p>
              </div>
            )}
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
